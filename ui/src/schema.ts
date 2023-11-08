import Ajv, { JSONSchemaType } from "ajv";
import { JSONSchemaBridge } from "uniforms-bridge-json-schema";

import prdocSchema from "../../prdoc_schema_user.json";

const ajv = new Ajv({
  allErrors: true,
  useDefaults: true,
  keywords: ["uniforms"],
});

function createValidator(schema: JSONSchemaType<unknown>) {
  const validator = ajv.compile(schema);

  return (model: Record<string, unknown>) => {
    validator(model);
    return validator.errors?.length ? { details: validator.errors } : null;
  };
}

const schemaValidator = createValidator(
  prdocSchema as unknown as JSONSchemaType<unknown>
);

export const bridge = new JSONSchemaBridge(prdocSchema, schemaValidator);
