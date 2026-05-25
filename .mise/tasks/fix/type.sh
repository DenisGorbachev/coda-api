#!/usr/bin/env -S usage bash
#USAGE arg "<needle>"
#USAGE arg "<replacement_file>"
#USAGE arg "<target_file>"

set -euo pipefail

needle=$usage_needle
replacement_file=$usage_replacement_file
target_file=$usage_target_file

needle_line=$(rg -n --no-filename "$needle" "$target_file" | cut -d: -f1)

# delete the #[derive(...)]
# delete the #[serde(deny_unknown_fields)]
delete_from_line=$((needle_line - 2))
delete_to_line=$((needle_line))
insert_after_line=$((needle_line - 3))

temp_file=$(mktemp)

sed -n "1,${insert_after_line}p" "$target_file" > "$temp_file"
cat "$replacement_file" >> "$temp_file"
sed -n "$((delete_to_line + 1)),\$p" "$target_file" >> "$temp_file"
mv "$temp_file" "$target_file"
