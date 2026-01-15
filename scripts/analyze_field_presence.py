#!/usr/bin/env python3
"""
Analyze field presence across IB FLEX XML files.
Outputs statistics on which fields are always present, sometimes present, or rarely present.
"""

import re
import sys
from collections import defaultdict
from pathlib import Path

def extract_elements(xml_content: str, element_name: str) -> list[dict[str, str]]:
    """Extract all elements of a given type and their attributes."""
    # Match self-closing elements like <Trade attr1="val1" attr2="val2" />
    pattern = rf'<{element_name}\s+([^>]*?)\s*/>'
    matches = re.findall(pattern, xml_content)

    elements = []
    for match in matches:
        attrs = {}
        # Extract attribute="value" pairs
        for attr_match in re.finditer(r'(\w+)="([^"]*)"', match):
            attrs[attr_match.group(1)] = attr_match.group(2)
        elements.append(attrs)

    return elements

def analyze_fields(elements: list[dict[str, str]]) -> dict[str, dict]:
    """Analyze field presence and value patterns."""
    total = len(elements)
    if total == 0:
        return {}

    field_stats = defaultdict(lambda: {
        'present': 0,
        'non_empty': 0,
        'sample_values': set()
    })

    for elem in elements:
        for field, value in elem.items():
            field_stats[field]['present'] += 1
            if value and value.strip():
                field_stats[field]['non_empty'] += 1
                # Keep up to 5 sample values
                if len(field_stats[field]['sample_values']) < 5:
                    field_stats[field]['sample_values'].add(value[:50])

    # Calculate percentages
    results = {}
    for field, stats in field_stats.items():
        pct_present = (stats['present'] / total) * 100
        pct_non_empty = (stats['non_empty'] / total) * 100 if stats['present'] > 0 else 0

        results[field] = {
            'present': stats['present'],
            'present_pct': pct_present,
            'non_empty': stats['non_empty'],
            'non_empty_pct': pct_non_empty,
            'samples': list(stats['sample_values'])[:3]
        }

    return results

def categorize_fields(results: dict, total: int) -> tuple[list, list, list]:
    """Categorize fields into always/sometimes/rarely present."""
    always = []  # 100% present AND non-empty
    sometimes = []  # 50-99% present or non-empty
    rarely = []  # <50% present or non-empty

    for field, stats in sorted(results.items()):
        pct = stats['non_empty_pct']
        if pct >= 99.9:
            always.append((field, stats))
        elif pct >= 50:
            sometimes.append((field, stats))
        else:
            rarely.append((field, stats))

    return always, sometimes, rarely

def print_category(name: str, fields: list, total: int):
    """Print a category of fields."""
    print(f"\n{'='*60}")
    print(f"{name} ({len(fields)} fields)")
    print('='*60)

    for field, stats in fields:
        samples = ', '.join(f'"{s}"' for s in stats['samples'][:2])
        print(f"  {field:40} {stats['non_empty_pct']:5.1f}%  ({stats['non_empty']}/{total})  {samples}")

def main():
    if len(sys.argv) < 2:
        xml_path = Path("/Users/clifton/code/convex/ib-flex/tmp/backfill-to-2026-01-13.xml")
    else:
        xml_path = Path(sys.argv[1])

    print(f"Reading {xml_path}...")
    xml_content = xml_path.read_text()

    # Analyze different element types
    element_types = ['Trade', 'OpenPosition', 'CashTransaction', 'CorporateAction']

    for elem_type in element_types:
        print(f"\n\n{'#'*60}")
        print(f"# {elem_type}")
        print('#'*60)

        elements = extract_elements(xml_content, elem_type)
        total = len(elements)
        print(f"Found {total} {elem_type} elements")

        if total == 0:
            continue

        results = analyze_fields(elements)
        always, sometimes, rarely = categorize_fields(results, total)

        print_category("ALWAYS PRESENT (>=99.9% non-empty) - Can be non-optional", always, total)
        print_category("SOMETIMES PRESENT (50-99%) - Conditional or truly optional", sometimes, total)
        print_category("RARELY PRESENT (<50%) - Truly optional", rarely, total)

if __name__ == '__main__':
    main()
