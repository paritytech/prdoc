import { useSearchParams } from "react-router-dom";

import Prdoc from "../components/Prdoc.tsx";

function MissingParameter({ name }: { name: string }) {
  return (
    <div>
      <h1>Error!</h1> <br />
      Missing parameter: {name}
    </div>
  );
}

function PrdocPage() {
  const [searchParams] = useSearchParams();

  const org = searchParams.get("org");
  if (!org) return <MissingParameter name="org" />;
  const repo = searchParams.get("repo");
  if (!repo) return <MissingParameter name="repo" />;
  const pull = searchParams.get("pull");
  if (!pull) return <MissingParameter name="pull" />;
  const branch = searchParams.get("branch");
  if (!branch) return <MissingParameter name="branch" />;

  return <Prdoc org={org} repo={repo} pull={pull} branch={branch}></Prdoc>;
}

export default PrdocPage;
