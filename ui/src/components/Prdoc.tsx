import { useState } from "react";
import { AutoForm } from "uniforms-semantic";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { coy } from "react-syntax-highlighter/dist/esm/styles/prism";

import yaml from "yaml";

import { bridge } from "../schema.ts";
import "./Prdoc.css";

function Prdoc({
  org,
  repo,
  pull,
  branch,
}: {
  org: string;
  repo: string;
  pull: string;
  branch: string;
}) {
  const [yamlValue, setYamlValue] = useState<string>("");

  function onValidate(model: unknown) {
    setYamlValue(
      yaml.stringify(model, {
        indent: 2,
        collectionStyle: "block",
        directives: true,
      })
    );
  }

  function onSubmit() {
    const branchEncoded = encodeURIComponent(branch);
    const prdocFileEncoded = encodeURIComponent(`prdoc/pr_${pull}.prdoc`);
    const yamlValueEncoded = encodeURIComponent(yamlValue);
    window.open(
      `https://github.com/${org}/${repo}/new/${branchEncoded}?filename=${prdocFileEncoded}&value=${yamlValueEncoded}`,
      "_blank"
    );
  }

  return (
    <>
      <h2>PRDOC generator</h2>
      <h3>{`Generating prdoc for /${org}/${repo}/${pull}. Branch name: ${branch}`}</h3>
      <div className="col">
        <AutoForm
          schema={bridge}
          onSubmit={onSubmit}
          onValidate={onValidate}
          validate="onChange"
        ></AutoForm>
      </div>
      <div className="col resultYaml">
        <SyntaxHighlighter language="yaml" style={coy}>
          {yamlValue}
        </SyntaxHighlighter>
      </div>
    </>
  );
}

export default Prdoc;
