#!/usr/bin/env bash
#
# release.sh - Automated release script for ib-flex Rust crate
#
# Usage: ./scripts/release.sh [major|minor|patch] [options]
#
# Options:
#   --dry-run        Preview changes without making them
#   -y, --yes        Non-interactive mode (auto-confirm all prompts)
#   --skip-tests     Skip cargo fmt/clippy/test checks
#   -h, --help       Show this help message
#
# Examples:
#   ./scripts/release.sh patch              # Bump patch version (0.1.0 -> 0.1.1)
#   ./scripts/release.sh minor --dry-run    # Preview minor version bump
#   ./scripts/release.sh major -y           # Major release, auto-confirm all

set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================
readonly SCRIPT_VERSION="1.0.0"
readonly DEFAULT_BRANCH="main"
readonly CARGO_TOML="Cargo.toml"
readonly CHANGELOG="CHANGELOG.md"
readonly GITHUB_REPO="clifton/ib-flex"

# ============================================================================
# GLOBAL STATE
# ============================================================================
DRY_RUN=false
AUTO_CONFIRM=false
SKIP_TESTS=false
VERSION_TYPE="patch"
CURRENT_VERSION=""
NEW_VERSION=""
PREVIOUS_TAG=""
FIRST_RELEASE=false

# Tracking what was done (for summary)
DID_COMMIT=false
DID_PUSH=false
DID_RELEASE=false

# Temp files for cleanup
TEMP_FILES=()

# ============================================================================
# COLOR OUTPUT (TTY-aware)
# ============================================================================
setup_colors() {
    if [[ -t 1 ]] && [[ -t 2 ]] && [[ "${TERM:-}" != "dumb" ]]; then
        RED='\033[0;31m'
        GREEN='\033[0;32m'
        YELLOW='\033[0;33m'
        BLUE='\033[0;34m'
        BOLD='\033[1m'
        DIM='\033[2m'
        NC='\033[0m'  # No Color
    else
        RED=''
        GREEN=''
        YELLOW=''
        BLUE=''
        BOLD=''
        DIM=''
        NC=''
    fi
}

# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================
log_info() {
    echo -e "${BLUE}::${NC} $1"
}

log_success() {
    echo -e "${GREEN}✓${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

log_error() {
    echo -e "${RED}✗${NC} $1" >&2
}

log_step() {
    echo ""
    echo -e "${BOLD}━━━ $1 ━━━${NC}"
}

# Confirm prompt (returns 0 for yes, 1 for no)
confirm() {
    local prompt="${1:-Continue?}"

    if $AUTO_CONFIRM; then
        log_info "$prompt [auto-confirmed]"
        return 0
    fi

    echo -ne "${YELLOW}?${NC} $prompt ${DIM}(y/N)${NC} "
    read -r response
    case "$response" in
        [yY]|[yY][eE][sS]) return 0 ;;
        *) return 1 ;;
    esac
}

# Create a temp file and track it for cleanup
make_temp() {
    local tmp
    tmp=$(mktemp)
    TEMP_FILES+=("$tmp")
    echo "$tmp"
}

# ============================================================================
# CLEANUP / ROLLBACK
# ============================================================================
cleanup() {
    local exit_code=$?

    # Clean up temp files
    for tmp in "${TEMP_FILES[@]:-}"; do
        rm -f "$tmp" 2>/dev/null || true
    done

    if [[ $exit_code -ne 0 ]] && ! $DRY_RUN; then
        echo ""
        log_warning "Script exited with error (code: $exit_code)"

        # Check if we have uncommitted changes to our tracked files
        if ! git diff --quiet -- "$CARGO_TOML" "$CHANGELOG" "Cargo.lock" 2>/dev/null; then
            log_warning "Uncommitted changes detected. Rolling back..."
            git checkout -- "$CARGO_TOML" "$CHANGELOG" "Cargo.lock" 2>/dev/null || true
            log_info "Rolled back changes to $CARGO_TOML, $CHANGELOG, Cargo.lock"
        fi

        # Warn about tag if it was created
        if [[ -n "${NEW_VERSION:-}" ]]; then
            local tag="v$NEW_VERSION"
            if git tag -l "$tag" | grep -q "$tag" 2>/dev/null; then
                log_warning "Tag $tag may have been created."
                log_info "To remove locally: git tag -d $tag"
                log_info "To remove from remote: git push origin :refs/tags/$tag"
            fi
        fi
    fi

    exit $exit_code
}

trap cleanup EXIT INT TERM

# ============================================================================
# HELP
# ============================================================================
show_help() {
    cat << 'EOF'
release.sh - Automated release script for ib-flex

USAGE:
    ./scripts/release.sh [VERSION_TYPE] [OPTIONS]

VERSION_TYPE:
    major           Bump major version (1.0.0 -> 2.0.0)
    minor           Bump minor version (0.1.0 -> 0.2.0)
    patch           Bump patch version (0.1.0 -> 0.1.1) [default]

OPTIONS:
    --dry-run       Preview all changes without making them
    -y, --yes       Non-interactive mode (auto-confirm all prompts)
    --skip-tests    Skip cargo fmt, clippy, and test checks
    -h, --help      Show this help message

EXAMPLES:
    ./scripts/release.sh patch              # Standard patch release
    ./scripts/release.sh minor --dry-run    # Preview minor release
    ./scripts/release.sh major -y           # Major release, no prompts
    ./scripts/release.sh --skip-tests       # Skip quality checks

REQUIREMENTS:
    - git
    - cargo (Rust toolchain)
    - gh (GitHub CLI, authenticated)
    - jq (JSON processor)

WORKFLOW:
    1. Pre-flight checks (branch, remote sync, clean worktree)
    2. Quality checks (fmt, clippy, test) unless --skip-tests
    3. Version bump in Cargo.toml
    4. Generate changelog from commits
    5. Update CHANGELOG.md
    6. Create git commit and tag
    7. Push to remote (prompted)
    8. Create GitHub release (prompted)
    9. GitHub Action publishes to crates.io automatically

EOF
    exit 0
}

# ============================================================================
# ARGUMENT PARSING
# ============================================================================
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            major|minor|patch)
                VERSION_TYPE="$1"
                shift
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            -y|--yes)
                AUTO_CONFIRM=true
                shift
                ;;
            --skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            -h|--help)
                show_help
                ;;
            *)
                log_error "Unknown argument: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done
}

# ============================================================================
# PRE-FLIGHT CHECKS
# ============================================================================
check_dependencies() {
    log_step "Checking dependencies"

    local missing=()

    command -v git >/dev/null 2>&1 || missing+=("git")
    command -v cargo >/dev/null 2>&1 || missing+=("cargo")
    command -v gh >/dev/null 2>&1 || missing+=("gh (GitHub CLI)")
    command -v jq >/dev/null 2>&1 || missing+=("jq")

    if [[ ${#missing[@]} -gt 0 ]]; then
        log_error "Missing required tools: ${missing[*]}"
        exit 1
    fi

    # Check gh authentication
    if ! gh auth status >/dev/null 2>&1; then
        log_error "GitHub CLI not authenticated"
        log_info "Run: gh auth login"
        exit 1
    fi

    log_success "All dependencies available"
}

check_branch() {
    # Check for detached HEAD
    if ! git symbolic-ref -q HEAD >/dev/null 2>&1; then
        log_error "HEAD is detached. Please checkout a branch."
        exit 1
    fi

    local current_branch
    current_branch=$(git branch --show-current)

    if [[ "$current_branch" != "$DEFAULT_BRANCH" ]]; then
        log_error "Must be on '$DEFAULT_BRANCH' branch to create a release"
        log_info "Current branch: $current_branch"
        log_info "Run: git checkout $DEFAULT_BRANCH"
        exit 1
    fi

    log_success "On $DEFAULT_BRANCH branch"
}

check_remote_sync() {
    log_info "Fetching from remote..."
    git fetch origin "$DEFAULT_BRANCH" --quiet

    local local_sha remote_sha base_sha
    local_sha=$(git rev-parse HEAD)
    remote_sha=$(git rev-parse "origin/$DEFAULT_BRANCH")
    base_sha=$(git merge-base HEAD "origin/$DEFAULT_BRANCH")

    if [[ "$local_sha" == "$remote_sha" ]]; then
        log_success "Branch is up to date with origin/$DEFAULT_BRANCH"
    elif [[ "$local_sha" == "$base_sha" ]]; then
        log_error "Local branch is behind remote"
        log_info "Run: git pull origin $DEFAULT_BRANCH"
        exit 1
    elif [[ "$remote_sha" == "$base_sha" ]]; then
        log_warning "Local branch is ahead of remote ($(git rev-list --count origin/$DEFAULT_BRANCH..HEAD) unpushed commits)"
        log_info "These commits will be included in the release"
    else
        log_error "Local and remote have diverged"
        log_info "Please resolve manually with: git pull --rebase origin $DEFAULT_BRANCH"
        exit 1
    fi
}

check_clean_worktree() {
    # Check for staged changes
    if ! git diff --cached --quiet; then
        log_error "Staged changes detected"
        log_info "Please commit or unstage them first"
        exit 1
    fi

    # Check for unstaged changes
    if ! git diff --quiet; then
        log_error "Unstaged changes detected"
        log_info "Please commit or stash them first"
        exit 1
    fi

    # Check for untracked files (warn only, excluding tmp/)
    local untracked
    untracked=$(git ls-files --others --exclude-standard 2>/dev/null | grep -v '^tmp/' | head -5 || true)
    if [[ -n "$untracked" ]]; then
        log_warning "Untracked files detected:"
        echo "$untracked" | sed 's/^/  /'
        if ! confirm "Continue with untracked files?"; then
            exit 1
        fi
    else
        log_success "Working directory is clean"
    fi
}

check_git_state() {
    log_step "Checking git state"
    check_branch
    check_remote_sync
    check_clean_worktree
}

run_quality_checks() {
    if $SKIP_TESTS; then
        log_step "Quality checks"
        log_warning "Skipping quality checks (--skip-tests)"
        return
    fi

    log_step "Running quality checks"

    log_info "Checking code formatting..."
    if ! cargo fmt --check --quiet 2>/dev/null; then
        log_error "Code is not formatted"
        log_info "Run: cargo fmt"
        exit 1
    fi
    log_success "Code formatting OK"

    log_info "Running clippy..."
    if ! cargo clippy --all-targets --quiet -- -D warnings 2>/dev/null; then
        log_error "Clippy has warnings"
        log_info "Run: cargo clippy --all-targets -- -D warnings"
        exit 1
    fi
    log_success "Clippy passed"

    log_info "Running tests..."
    if ! cargo test --quiet 2>/dev/null; then
        log_error "Tests failed"
        exit 1
    fi
    log_success "All tests passed"
}

# ============================================================================
# VERSION MANAGEMENT
# ============================================================================
get_current_version() {
    CURRENT_VERSION=$(grep -E '^version\s*=' "$CARGO_TOML" | head -1 | sed 's/.*"\([^"]*\)".*/\1/')

    if [[ -z "$CURRENT_VERSION" ]] || [[ ! "$CURRENT_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        log_error "Could not parse version from $CARGO_TOML"
        log_info "Expected format: version = \"X.Y.Z\""
        exit 1
    fi
}

calculate_new_version() {
    local major minor patch
    IFS='.' read -r major minor patch <<< "$CURRENT_VERSION"

    case "$VERSION_TYPE" in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
    esac

    NEW_VERSION="${major}.${minor}.${patch}"
}

get_previous_tag() {
    PREVIOUS_TAG=$(git tag -l 'v*.*.*' --sort=-v:refname 2>/dev/null | head -1 || true)

    if [[ -z "$PREVIOUS_TAG" ]]; then
        log_warning "No previous release tags found - this will be the first release"
        FIRST_RELEASE=true
        # Use the first commit as the starting point
        PREVIOUS_TAG=$(git rev-list --max-parents=0 HEAD 2>/dev/null | head -1)
    fi
}

check_tag_exists() {
    local tag="v$NEW_VERSION"

    # Check local
    if git tag -l "$tag" | grep -q "^$tag$" 2>/dev/null; then
        log_error "Tag $tag already exists locally"
        log_info "To delete: git tag -d $tag"
        exit 1
    fi

    # Check remote
    if git ls-remote --tags origin "$tag" 2>/dev/null | grep -q "$tag"; then
        log_error "Tag $tag already exists on remote"
        exit 1
    fi
}

update_cargo_toml() {
    if $DRY_RUN; then
        log_info "[DRY RUN] Would update $CARGO_TOML: $CURRENT_VERSION -> $NEW_VERSION"
        return
    fi

    # macOS and GNU sed compatibility
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
    else
        sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
    fi

    # Verify the change worked
    local updated
    updated=$(grep -E '^version\s*=' "$CARGO_TOML" | head -1 | sed 's/.*"\([^"]*\)".*/\1/')
    if [[ "$updated" != "$NEW_VERSION" ]]; then
        log_error "Failed to update version in $CARGO_TOML"
        exit 1
    fi

    # Update Cargo.lock
    cargo update --package ib-flex --quiet 2>/dev/null || cargo generate-lockfile --quiet 2>/dev/null || true

    log_success "Updated $CARGO_TOML and Cargo.lock"
}

# ============================================================================
# CHANGELOG GENERATION
# ============================================================================
get_commits_since_tag() {
    local since="$1"
    local range

    if $FIRST_RELEASE; then
        range="HEAD"
    else
        range="${since}..HEAD"
    fi

    # Get commits with hash|subject format, excluding merge commits
    git log "$range" --pretty=format:'%h|%s' --no-merges 2>/dev/null || true
}

categorize_commit() {
    local subject="$1"
    local category="other"

    # Match conventional commit prefixes
    # Use case statement for better compatibility across bash versions
    case "$subject" in
        feat:*|feat\(*) category="features" ;;
        fix:*|fix\(*) category="fixes" ;;
        perf:*|perf\(*) category="performance" ;;
        docs:*|docs\(*) category="docs" ;;
        refactor:*|refactor\(*) category="refactor" ;;
        test:*|test\(*) category="tests" ;;
        chore:*|chore\(*) category="chore" ;;
        ci:*|ci\(*) category="ci" ;;
        build:*|build\(*) category="build" ;;
        style:*|style\(*) category="style" ;;
        *)
            # Not a conventional commit, will try keyword inference below
            :
            ;;
    esac

    # If still "other", try keyword inference
    if [[ "$category" == "other" ]]; then
        # Try to infer from keywords for non-conventional commits
        local lower_subject
        lower_subject=$(echo "$subject" | tr '[:upper:]' '[:lower:]')
        case "$lower_subject" in
            *add*|*implement*|*new\ *|*create*|*introduce*)
                category="features"
                ;;
            *fix*|*bug*|*resolve*|*correct*|*repair*)
                category="fixes"
                ;;
        esac
    fi

    echo "$category"
}

clean_commit_subject() {
    local subject="$1"
    # Remove conventional commit prefix but keep the rest, then trim leading/trailing whitespace
    echo "$subject" | sed -E 's/^(feat|fix|docs|perf|refactor|test|chore|ci|build|style)(\([^)]*\))?:\s*//' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//'
}

extract_pr_number() {
    local subject="$1"
    # Extract PR number from subject like "(#123)"
    # Use grep/sed for compatibility across bash versions
    echo "$subject" | grep -oE '\(#[0-9]+\)' | head -1 | tr -d '(#)' || true
}

generate_changelog_section() {
    local release_date
    release_date=$(date +%Y-%m-%d)

    local features="" fixes="" performance="" docs="" changed=""
    local seen_contributors=""

    while IFS='|' read -r hash subject; do
        [[ -z "$hash" ]] && continue

        # Skip version bump commits
        case "$subject" in
            bump*|Bump*|chore:*[Rr]elease*|chore:*[Vv]ersion*)
                continue
                ;;
        esac

        local category pr_num clean_subject entry author
        category=$(categorize_commit "$subject")
        pr_num=$(extract_pr_number "$subject")
        clean_subject=$(clean_commit_subject "$subject")

        # Build the entry
        entry="- $clean_subject"
        if [[ -n "$pr_num" ]]; then
            entry="$entry ([#$pr_num](https://github.com/$GITHUB_REPO/pull/$pr_num))"

            # Try to get author from PR (skip API call in dry run for speed)
            if ! $DRY_RUN; then
                author=$(gh api "repos/$GITHUB_REPO/pulls/$pr_num" --jq '.user.login' 2>/dev/null || true)
                if [[ -n "$author" ]]; then
                    # Track unique contributors using string search
                    if [[ "$seen_contributors" != *"@$author"* ]]; then
                        seen_contributors="$seen_contributors @$author"
                    fi
                fi
            fi
        fi

        # Add to appropriate category
        case "$category" in
            features) features="$features$entry"$'\n' ;;
            fixes) fixes="$fixes$entry"$'\n' ;;
            performance) performance="$performance$entry"$'\n' ;;
            docs) docs="$docs$entry"$'\n' ;;
            *) changed="$changed$entry"$'\n' ;;
        esac
    done < <(get_commits_since_tag "$PREVIOUS_TAG")

    # Build the section
    local section=""
    section+="## [$NEW_VERSION] - $release_date"$'\n'
    section+=""$'\n'

    if [[ -n "$features" ]]; then
        section+="### Added"$'\n'
        section+="$features"$'\n'
    fi

    if [[ -n "$fixes" ]]; then
        section+="### Fixed"$'\n'
        section+="$fixes"$'\n'
    fi

    if [[ -n "$performance" ]]; then
        section+="### Performance"$'\n'
        section+="$performance"$'\n'
    fi

    if [[ -n "$docs" ]]; then
        section+="### Documentation"$'\n'
        section+="$docs"$'\n'
    fi

    if [[ -n "$changed" ]]; then
        section+="### Changed"$'\n'
        section+="$changed"$'\n'
    fi

    # If no changes categorized, add a note
    if [[ -z "$features" ]] && [[ -z "$fixes" ]] && [[ -z "$performance" ]] && [[ -z "$docs" ]] && [[ -z "$changed" ]]; then
        section+="### Changed"$'\n'
        section+="- Minor improvements and maintenance"$'\n'
        section+=""$'\n'
    fi

    echo "$section"
}

generate_github_release_notes() {
    local release_notes=""
    local features="" fixes="" changed=""
    local seen_contributors=""
    local contributors_list=""

    while IFS='|' read -r hash subject; do
        [[ -z "$hash" ]] && continue

        # Skip version bump commits
        case "$subject" in
            bump*|Bump*|chore:*[Rr]elease*|chore:*[Vv]ersion*)
                continue
                ;;
        esac

        local category pr_num clean_subject entry author
        category=$(categorize_commit "$subject")
        pr_num=$(extract_pr_number "$subject")
        clean_subject=$(clean_commit_subject "$subject")

        # Build rich entry for GitHub
        entry="- $clean_subject"
        if [[ -n "$pr_num" ]]; then
            entry="$entry (#$pr_num)"

            # Get author
            author=$(gh api "repos/$GITHUB_REPO/pulls/$pr_num" --jq '.user.login' 2>/dev/null || true)
            if [[ -n "$author" ]]; then
                entry="$entry by @$author"
                # Track unique contributors using string search
                if [[ "$seen_contributors" != *"@$author"* ]]; then
                    seen_contributors="$seen_contributors @$author"
                    contributors_list="$contributors_list- @$author"$'\n'
                fi
            fi
        fi

        case "$category" in
            features) features="$features$entry"$'\n' ;;
            fixes) fixes="$fixes$entry"$'\n' ;;
            *) changed="$changed$entry"$'\n' ;;
        esac
    done < <(get_commits_since_tag "$PREVIOUS_TAG")

    release_notes+="## What's Changed"$'\n'
    release_notes+=""$'\n'

    if [[ -n "$features" ]]; then
        release_notes+="### New Features"$'\n'
        release_notes+="$features"$'\n'
    fi

    if [[ -n "$fixes" ]]; then
        release_notes+="### Bug Fixes"$'\n'
        release_notes+="$fixes"$'\n'
    fi

    if [[ -n "$changed" ]]; then
        release_notes+="### Other Changes"$'\n'
        release_notes+="$changed"$'\n'
    fi

    if [[ -n "$contributors_list" ]]; then
        release_notes+="### Contributors"$'\n'
        release_notes+="$contributors_list"$'\n'
    fi

    # Add full changelog link
    if $FIRST_RELEASE; then
        release_notes+="**Full Changelog**: https://github.com/$GITHUB_REPO/commits/v$NEW_VERSION"$'\n'
    else
        release_notes+="**Full Changelog**: https://github.com/$GITHUB_REPO/compare/$PREVIOUS_TAG...v$NEW_VERSION"$'\n'
    fi

    echo "$release_notes"
}

update_changelog_file() {
    local new_section="$1"

    if $DRY_RUN; then
        log_info "[DRY RUN] Would update $CHANGELOG"
        return
    fi

    if [[ ! -f "$CHANGELOG" ]]; then
        log_warning "$CHANGELOG not found, creating new file"
        cat > "$CHANGELOG" << 'EOF'
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

EOF
    fi

    # Check if [Unreleased] section exists
    if ! grep -q '## \[Unreleased\]' "$CHANGELOG"; then
        log_warning "$CHANGELOG missing [Unreleased] section"
        # Prepend the standard header
        local tmp
        tmp=$(make_temp)
        cat > "$tmp" << 'EOF'
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

EOF
        cat "$CHANGELOG" >> "$tmp"
        mv "$tmp" "$CHANGELOG"
    fi

    # Insert new section after [Unreleased]
    # Write the new section to a temp file (avoids awk multi-line variable issues)
    local section_file
    section_file=$(make_temp)
    echo "$new_section" > "$section_file"

    local tmp
    tmp=$(make_temp)

    # Use awk to insert the new section after [Unreleased]
    awk -v section_file="$section_file" '
    /^## \[Unreleased\]/ {
        print
        print ""
        # Read and print the section from file
        while ((getline line < section_file) > 0) {
            print line
        }
        close(section_file)
        # Skip any existing unreleased content until next ## header
        while ((getline line) > 0) {
            if (line ~ /^## \[/) {
                print line
                break
            }
        }
        next
    }
    { print }
    ' "$CHANGELOG" > "$tmp"

    mv "$tmp" "$CHANGELOG"
    log_success "Updated $CHANGELOG"
}

# ============================================================================
# RELEASE ACTIONS
# ============================================================================
create_git_commit() {
    if $DRY_RUN; then
        log_info "[DRY RUN] Would create commit: chore: Release v$NEW_VERSION"
        return
    fi

    git add "$CARGO_TOML" "Cargo.lock" "$CHANGELOG"
    git commit -m "chore: Release v$NEW_VERSION"
    DID_COMMIT=true
    log_success "Created commit for v$NEW_VERSION"
}

create_git_tag() {
    local tag="v$NEW_VERSION"

    if $DRY_RUN; then
        log_info "[DRY RUN] Would create tag: $tag"
        return
    fi

    git tag -a "$tag" -m "Release $NEW_VERSION"
    log_success "Created tag $tag"
}

push_to_remote() {
    if $DRY_RUN; then
        log_info "[DRY RUN] Would push to remote"
        return
    fi

    if ! confirm "Push commit and tag to remote?"; then
        log_info "Skipped pushing to remote"
        echo ""
        log_info "To push manually:"
        echo "  git push && git push origin v$NEW_VERSION"
        return
    fi

    git push origin "$DEFAULT_BRANCH"
    git push origin "v$NEW_VERSION"
    DID_PUSH=true
    log_success "Pushed to remote"
}

create_github_release() {
    if $DRY_RUN; then
        log_info "[DRY RUN] Would create GitHub release"
        return
    fi

    if ! $DID_PUSH; then
        log_warning "Changes not pushed to remote - skipping GitHub release"
        return
    fi

    if ! confirm "Create GitHub release?"; then
        log_info "Skipped GitHub release"
        return
    fi

    local release_notes
    release_notes=$(generate_github_release_notes)

    local tmp_notes
    tmp_notes=$(make_temp)
    echo "$release_notes" > "$tmp_notes"

    gh release create "v$NEW_VERSION" \
        --title "v$NEW_VERSION" \
        --notes-file "$tmp_notes" \
        --target "$DEFAULT_BRANCH"

    DID_RELEASE=true
    log_success "Created GitHub release v$NEW_VERSION"
}


# ============================================================================
# SUMMARY
# ============================================================================
print_summary() {
    echo ""
    log_step "Release Summary"
    echo ""
    echo -e "  Version:      ${YELLOW}$CURRENT_VERSION${NC} -> ${GREEN}$NEW_VERSION${NC} (${VERSION_TYPE})"
    echo -e "  Tag:          v$NEW_VERSION"

    if $DRY_RUN; then
        echo ""
        echo -e "  ${YELLOW}This was a dry run - no changes were made${NC}"
        echo ""
        echo "  To perform the actual release, run without --dry-run"
        return
    fi

    echo ""
    echo -e "  Committed:    $(if $DID_COMMIT; then echo "${GREEN}Yes${NC}"; else echo "${DIM}No${NC}"; fi)"
    echo -e "  Pushed:       $(if $DID_PUSH; then echo "${GREEN}Yes${NC}"; else echo "${DIM}No${NC}"; fi)"
    echo -e "  GH Release:   $(if $DID_RELEASE; then echo "${GREEN}Yes${NC}"; else echo "${DIM}No${NC}"; fi)"

    if ! $DID_PUSH; then
        echo ""
        echo "  To complete the release:"
        echo "    git push && git push origin v$NEW_VERSION"
    fi

    if $DID_RELEASE; then
        echo ""
        echo "  Release URL: https://github.com/$GITHUB_REPO/releases/tag/v$NEW_VERSION"
    fi

    if $DID_PUSH; then
        echo ""
        echo "  crates.io publish will be handled by GitHub Action"
    fi
}

# ============================================================================
# MAIN
# ============================================================================
main() {
    setup_colors
    parse_args "$@"

    echo ""
    echo -e "${BOLD}ib-flex Release Script v$SCRIPT_VERSION${NC}"
    if $DRY_RUN; then
        echo -e "${YELLOW}(DRY RUN MODE - no changes will be made)${NC}"
    fi

    # Pre-flight
    check_dependencies
    check_git_state
    run_quality_checks

    # Version management
    log_step "Version management"
    get_current_version
    calculate_new_version
    get_previous_tag
    check_tag_exists

    echo ""
    echo -e "  Current version: ${YELLOW}$CURRENT_VERSION${NC}"
    echo -e "  New version:     ${GREEN}$NEW_VERSION${NC} (${VERSION_TYPE} bump)"
    if ! $FIRST_RELEASE; then
        echo -e "  Previous tag:    $PREVIOUS_TAG"
    else
        echo -e "  ${DIM}(First release - no previous tag)${NC}"
    fi
    echo ""

    if ! $AUTO_CONFIRM && ! $DRY_RUN; then
        if ! confirm "Proceed with release?"; then
            log_info "Release cancelled"
            exit 0
        fi
    fi

    # Generate changelog
    log_step "Generating changelog"
    local changelog_section
    changelog_section=$(generate_changelog_section)

    echo ""
    echo -e "${DIM}--- CHANGELOG Preview ---${NC}"
    echo "$changelog_section" | head -30
    echo -e "${DIM}--- End Preview ---${NC}"
    echo ""

    # Apply changes
    log_step "Applying changes"
    update_cargo_toml
    update_changelog_file "$changelog_section"

    # Git operations
    log_step "Git operations"
    create_git_commit
    create_git_tag

    # Release operations
    log_step "Release operations"
    push_to_remote
    create_github_release

    # Summary
    print_summary

    echo ""
    log_success "Release script completed!"
}

main "$@"
