#!/usr/bin/env bash

set -xeuo pipefail

SPEC="coda.openapi.json"
LIB="src/lib.rs"
GEN="src/gen.rs"
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
jq '
    def column_format_type($types):
        {
            "type": "string",
            "enum": $types
        };
    def append_unique($value):
        if index($value) == null then
            . + [$value]
        else
            .
        end;
    def column_format($name; $description; $types; $required; $properties):
        .components.schemas[$name] = {
            "x-schema-name": $name,
            "description": $description,
            "type": "object",
            "additionalProperties": false,
            "required": (["type", "isArray"] + $required),
            "properties": (
                .components.schemas.SimpleColumnFormat.properties
                + {
                    "type": column_format_type($types)
                }
                + $properties
            )
        };
    del(.paths["/docs/{docId}/hooks/automation/{ruleId}"].post.requestBody.content["application/x-www-form-urlencoded","text/plain"])
    # Workaround: Coda returns docSize.baseTableCount, but the OpenAPI spec does not declare it yet.
    | .components.schemas.DocSize.properties.baseTableCount = {
        "type": "number",
        "description": "The number of base tables contained within the doc.",
        "example": 25
    }
    # Workaround: Coda returns Page.contentType="table", but the OpenAPI PageType enum does not declare it yet.
    | .components.schemas.PageTypeEnum.enum |= append_unique("table")
    | .components.schemas.PageTypeEnum["x-tsEnumNames"] |= append_unique("Table")
    # Workaround: progenitor turns the sourceDoc allOf wrapper into an empty enum.
    | .components.schemas.Doc.properties.sourceDoc = {
        "$ref": "#/components/schemas/DocReference"
    }
    | .components.schemas.DocumentCreationResult.properties.sourceDoc = {
        "$ref": "#/components/schemas/DocReference"
    }
    # Workaround: progenitor turns the Table.filter allOf wrapper into an empty enum.
    | .components.schemas.Table.properties.filter = {
        "$ref": "#/components/schemas/FormulaDetail"
    }
    # Workaround: progenitor turns column-format allOf wrappers into empty enums.
    | .components.schemas.SimpleColumnFormat.properties.type = column_format_type([
        "text",
        "image",
        "attachments",
        "packObject",
        "reaction",
        "canvas",
        "other"
    ])
    | column_format(
        "ButtonColumnFormat";
        "Format of a button column.";
        ["button"];
        [];
        {
            "label": {
                "type": "string",
                "description": "Label formula for the button.",
                "example": "Click me"
            },
            "disableIf": {
                "type": "string",
                "description": "DisableIf formula for the button.",
                "example": "False()"
            },
            "action": {
                "type": "string",
                "description": "Action formula for the button.",
                "example": "OpenUrl(\"www.google.com\")"
            }
        }
    )
    | column_format(
        "CheckboxColumnFormat";
        "Format of a checkbox column.";
        ["checkbox"];
        ["displayType"];
        {
            "displayType": {
                "$ref": "#/components/schemas/CheckboxDisplayType"
            }
        }
    )
    | column_format(
        "DateColumnFormat";
        "Format of a date column.";
        ["date"];
        [];
        {
            "format": {
                "type": "string",
                "description": "A format string using Moment syntax: https://momentjs.com/docs/#/displaying/",
                "example": "YYYY-MM-DD"
            }
        }
    )
    | column_format(
        "DateTimeColumnFormat";
        "Format of a date column.";
        ["dateTime"];
        [];
        {
            "dateFormat": {
                "type": "string",
                "description": "A format string using Moment syntax: https://momentjs.com/docs/#/displaying/",
                "example": "YYYY-MM-DD"
            },
            "timeFormat": {
                "type": "string",
                "description": "A format string using Moment syntax: https://momentjs.com/docs/#/displaying/",
                "example": "h:mm:ss A"
            }
        }
    )
    | column_format(
        "DurationColumnFormat";
        "Format of a duration column.";
        ["duration"];
        [];
        {
            "precision": {
                "type": "integer",
                "example": 2
            },
            "maxUnit": {
                "$ref": "#/components/schemas/DurationUnit"
            }
        }
    )
    | column_format(
        "EmailColumnFormat";
        "Format of an email column.";
        ["email"];
        [];
        {
            "display": {
                "$ref": "#/components/schemas/EmailDisplayType"
            },
            "autocomplete": {
                "type": "boolean"
            }
        }
    )
    | column_format(
        "LinkColumnFormat";
        "Format of a link column.";
        ["link"];
        [];
        {
            "display": {
                "$ref": "#/components/schemas/LinkDisplayType"
            },
            "force": {
                "type": "boolean",
                "description": "Force embeds to render on the client instead of the server (for sites that require user login).",
                "example": true
            }
        }
    )
    | column_format(
        "CurrencyColumnFormat";
        "Format of a currency column.";
        ["currency"];
        [];
        {
            "currencyCode": {
                "type": "string",
                "description": "The currency symbol",
                "example": "$"
            },
            "precision": {
                "type": "integer",
                "minimum": 0,
                "maximum": 10,
                "description": "The decimal precision.",
                "example": 2
            },
            "format": {
                "$ref": "#/components/schemas/CurrencyFormatType"
            }
        }
    )
    | column_format(
        "ImageReferenceColumnFormat";
        "Format of an image reference column.";
        ["imageReference"];
        ["width", "height", "style"];
        {
            "width": {
                "$ref": "#/components/schemas/NumberOrNumberFormula"
            },
            "height": {
                "$ref": "#/components/schemas/NumberOrNumberFormula"
            },
            "style": {
                "$ref": "#/components/schemas/ImageShapeStyle"
            }
        }
    )
    | column_format(
        "NumericColumnFormat";
        "Format of a numeric column.";
        ["number", "percent"];
        [];
        {
            "precision": {
                "type": "integer",
                "minimum": 0,
                "maximum": 10,
                "description": "The decimal precision.",
                "example": 2
            },
            "useThousandsSeparator": {
                "type": "boolean",
                "description": "Whether to use a thousands separator (like \",\") to format the numeric value.",
                "example": true
            }
        }
    )
    | column_format(
        "ReferenceColumnFormat";
        "Format of a column that refers to another table.";
        ["person", "lookup"];
        [];
        {
            "table": {
                "$ref": "#/components/schemas/TableReference"
            }
        }
    )
    | column_format(
        "SelectColumnFormat";
        "Format of a select column.";
        ["select"];
        [];
        {
            "options": {
                "type": "array",
                "description": "For select format columns, the list of available options. Only returned for select lists that used a fixed set of options. Returns the first 5000 options.",
                "items": {
                    "$ref": "#/components/schemas/SelectOption"
                }
            }
        }
    )
    | column_format(
        "ScaleColumnFormat";
        "Format of a numeric column that renders as a scale, like star ratings.";
        ["scale"];
        ["maximum", "icon"];
        {
            "maximum": {
                "type": "number",
                "description": "The maximum number allowed for this scale.",
                "example": 5
            },
            "icon": {
                "$ref": "#/components/schemas/IconSet"
            }
        }
    )
    | column_format(
        "SliderColumnFormat";
        "Format of a numeric column that renders as a slider.";
        ["slider"];
        [];
        {
            "minimum": {
                "$ref": "#/components/schemas/NumberOrNumberFormula"
            },
            "maximum": {
                "$ref": "#/components/schemas/NumberOrNumberFormula"
            },
            "step": {
                "$ref": "#/components/schemas/NumberOrNumberFormula"
            },
            "displayType": {
                "$ref": "#/components/schemas/SliderDisplayType"
            },
            "showValue": {
                "type": "boolean",
                "description": "Whether the underyling numeric value is also displayed.",
                "example": true
            }
        }
    )
    | column_format(
        "TimeColumnFormat";
        "Format of a time column.";
        ["time"];
        [];
        {
            "format": {
                "type": "string",
                "description": "A format string using Moment syntax: https://momentjs.com/docs/#/displaying/",
                "example": "h:mm:ss A"
            }
        }
    )
    # Workaround: progenitor turns the PersonValue allOf wrapper into an empty enum.
    | .components.schemas.PersonValue = {
        "x-schema-name": "PersonValue",
        "description": "A named reference to a person, where the person is identified by email address.",
        "type": "object",
        "additionalProperties": false,
        "required": [
            "@context",
            "@type",
            "name"
        ],
        "properties": {
            "@context": {
                "type": "string",
                "description": "A url describing the schema context for this object, typically \"http://schema.org/\".",
                "example": "http://schema.org/"
            },
            "@type": {
                "type": "string",
                "enum": [
                    "Person"
                ],
                "x-tsType": "LinkedDataType.Person"
            },
            "additionalType": {
                "type": "string",
                "description": "An identifier of additional type info specific to Coda that may not be present in a schema.org taxonomy."
            },
            "name": {
                "type": "string",
                "description": "The full name of the person.",
                "example": "Alice Atkins"
            },
            "email": {
                "type": "string",
                "description": "The email address of the person.",
                "example": "alice@atkins.com"
            }
        }
    }
' "$SPEC" \
    | jq . > "$SPEC_FORMATTED"
mv "$SPEC_FORMATTED" "$SPEC"

cargo progenitor -i $SPEC -o $TEMP -n coda_api -v 0.1.0
mkdir -p src
rm -f "$GEN"
# #![allow(unreachable_code)] is needed to work around .clone() calls on empty enums
# this line must be at the top of the file
echo "#![allow(unreachable_code)]" >> $GEN
echo "#![allow(clippy::absolute_paths)]" >> $GEN
cat $TEMP/$LIB >> $GEN
$SDF '///!' '/// !' $GEN
$SDF '<span>' '' $GEN
$SDF 'go/<name>/<var1>/<var2>' 'go/{name}/{var1}/{var2}' $GEN
$SDF '"http://schema.org/"' '"<http://schema.org/>"' $GEN
$SDF 'https://coda.io/trust/tos' '<https://coda.io/trust/tos>' $GEN
$SDF '\n/// border' ' border' $GEN
$SDF '    Your doc ID' 'Your doc ID' $GEN
$SDF "elided_named_lifetimes" "mismatched_lifetime_syntaxes" $GEN
$SDF "pub struct Client {" "pub struct RawClient {" $GEN
$SDF "impl Client {" "impl RawClient {" $GEN
$SDF "for Client {" "for RawClient {" $GEN
$SDF "for &Client {" "for &RawClient {" $GEN
$SDF "pub use super::Client;" "pub use super::RawClient;" $GEN

# `fix` must be executed before `fix:type`
mise run fix
mise run fix:type "pub enum PushButtonResult" snippets/PushButtonResult "$GEN"
mise run test
