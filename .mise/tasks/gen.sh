#!/usr/bin/env bash

set -xeuo pipefail

SPEC="coda.openapi.json"
LIB="src/lib.rs"
SDF="sd --fixed-strings"
TEMP=$(mktemp -d)
SPEC_FORMATTED="$TEMP/coda.openapi.formatted.json"

# download the spec
curl https://coda.io/apis/v1/openapi.json > $SPEC

# fix & format the spec
for ENUM in PageType TableType ControlType; do
    $SDF "$ENUM" "${ENUM}Enum" $SPEC
done
for FIELD in aiCreditsChat aiCreditsBlock aiCreditsColumn aiCreditsAssistant aiCreditsReviewer aiCredits; do
    $SDF "$FIELD," "$FIELD" $SPEC
done
# add the "json" language tag
$SDF '[JSON-LD](https://json-ld.org/) format.\n\n```\n  // Currency' '[JSON-LD](https://json-ld.org/) format.\n\n```json\n  // Currency' $SPEC
jq 'del(.paths["/docs/{docId}/hooks/automation/{ruleId}"].post.requestBody.content["application/x-www-form-urlencoded","text/plain"])' "$SPEC" \
    | jq . > "$SPEC_FORMATTED"
mv "$SPEC_FORMATTED" "$SPEC"

cargo progenitor -i $SPEC -o $TEMP -n coda_api -v 0.1.0
mkdir -p src
rm -f "$LIB"
# #![allow(unreachable_code)] is needed to work around .clone() calls on empty enums
# this line must be at the top of the file
echo "#![allow(unreachable_code)]" >> $LIB
cat $TEMP/$LIB >> $LIB
$SDF '///!' '/// !' $LIB
$SDF '<span>' '' $LIB
$SDF 'go/<name>/<var1>/<var2>' 'go/{name}/{var1}/{var2}' $LIB
$SDF '"http://schema.org/"' '"<http://schema.org/>"' $LIB
$SDF 'https://coda.io/trust/tos' '<https://coda.io/trust/tos>' $LIB
$SDF '\n/// border' ' border' $LIB
$SDF '    Your doc ID' 'Your doc ID' $LIB
$SDF "elided_named_lifetimes" "mismatched_lifetime_syntaxes" $LIB
$SDF "pub struct Client {" "pub struct RawClient {" $LIB
$SDF "impl Client {" "impl RawClient {" $LIB
$SDF "for Client {" "for RawClient {" $LIB
$SDF "for &Client {" "for &RawClient {" $LIB
$SDF "pub use super::Client;" "pub use super::RawClient;" $LIB
# echo "mod metadata;" >> $LIB
# echo "pub use metadata::*;" >> $LIB
echo "mod ext;" >> $LIB
echo "pub use ext::*;" >> $LIB
echo "mod client;" >> $LIB
echo "pub use client::*;" >> $LIB
echo "mod limiter;" >> $LIB
echo "pub use limiter::*;" >> $LIB
echo "#[cfg(test)] pub mod test;" >> $LIB

# `fix` must be executed before `fix:type`
mise run fix
mise run fix:type "pub enum PushButtonResult" snippets/PushButtonResult src/lib.rs
mise run test
