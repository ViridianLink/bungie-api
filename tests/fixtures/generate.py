#!/usr/bin/env python3
"""Generate JSON fixtures from Bungie's OpenAPI spec.

Every property of each schema is populated, following Bungie's wire format
(64-bit integers as strings, enum-keyed dictionaries keyed by name), so that
deserializing the fixtures with the `strict` feature checks the models against
the spec.

Usage: python3 tests/fixtures/generate.py path/to/openapi.json
The spec lives at https://github.com/Bungie-net/api/blob/master/openapi.json
"""

import json
import pathlib
import sys

FIXTURES = {
    "profile_response.json": "Destiny.Responses.DestinyProfileResponse",
    "post_game_carnage_report.json": "Destiny.HistoricalStats.DestinyPostGameCarnageReportData",
    "activity_history.json": "Destiny.HistoricalStats.DestinyActivityHistoryResults",
    "user_info_card.json": "User.UserInfoCard",
    "manifest.json": "Destiny.Config.DestinyManifest",
}


# Fields Bungie sends that the spec does not document, keyed by schema name.
UNDOCUMENTED = {
    "Destiny.Entities.Items.DestinyItemComponent": {"dismantlePermission": 1},
    "Destiny.DestinyActivityDifficultyTierComponent": {"isEnabled": True},
    "Destiny.Definitions.DestinyActivityRewardItem": {
        "visibilityUnlockExpression": {
            "steps": [{"stepOperator": 1, "unlockHash": 1, "value": 0}],
            "scope": 1,
        },
    },
}


def main() -> None:
    spec = json.loads(pathlib.Path(sys.argv[1]).read_text())
    schemas = spec["components"]["schemas"]

    def ref_name(ref: str) -> str:
        return ref.rsplit("/", 1)[-1]

    def scalar(prop: dict):
        fmt = prop.get("format")
        enum_ref = prop.get("x-enum-reference")
        if enum_ref:
            values = schemas[ref_name(enum_ref["$ref"])]["x-enum-values"]
            return int(values[-1]["numericValue"])
        kind = prop.get("type")
        if kind == "integer":
            return "1" if fmt in ("int64", "uint64") else 1
        if kind == "number":
            return 1.5
        if kind == "boolean":
            return True
        if kind == "string":
            if fmt == "date-time":
                return "2024-01-01T00:00:00Z"
            if fmt == "byte":
                return 1
            return "text"
        raise ValueError(prop)

    def key(prop: dict) -> str:
        key_schema = prop.get("x-dictionary-key", {})
        enum_ref = key_schema.get("x-enum-reference")
        if key_schema.get("type") == "string" and enum_ref:
            values = schemas[ref_name(enum_ref["$ref"])]["x-enum-values"]
            return values[-1]["identifier"]
        if key_schema.get("type") == "integer":
            return "1"
        return "key"

    def value(prop: dict, stack: tuple):
        if "allOf" in prop:
            prop = prop["allOf"][0]
        if "$ref" in prop:
            name = ref_name(prop["$ref"])
            schema = schemas[name]
            if "x-enum-values" in schema:
                return int(schema["x-enum-values"][-1]["numericValue"])
            if name in stack:
                return None
            return obj(schema, stack + (name,))
        if prop.get("type") == "array":
            item = value(prop["items"], stack)
            return [] if item is None else [item]
        if prop.get("type") == "object" and "additionalProperties" in prop:
            item = value(prop["additionalProperties"], stack)
            return {} if item is None else {key(prop): item}
        return scalar(prop)

    def obj(schema: dict, stack: tuple) -> dict:
        out = dict(UNDOCUMENTED.get(stack[-1], {}))
        for name, prop in schema.get("properties", {}).items():
            item = value(prop, stack)
            if item is not None:
                out[name] = item
        return out

    out_dir = pathlib.Path(__file__).parent
    for file, schema in FIXTURES.items():
        data = obj(schemas[schema], (schema,))
        (out_dir / file).write_text(json.dumps(data, indent=1) + "\n")


if __name__ == "__main__":
    main()
