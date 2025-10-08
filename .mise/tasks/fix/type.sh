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
insert_from_line=$((needle_line - 3))

ed -s "$target_file" <<ED
$delete_from_line,$delete_to_line d
$insert_from_line r $replacement_file
wq
ED
