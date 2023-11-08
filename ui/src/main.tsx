import ReactDOM from "react-dom/client";

import { RouterProvider } from "react-router-dom";
import "semantic-ui-css/semantic.min.css";
import "./index.css";
import { router } from "./router.tsx";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <RouterProvider router={router} />
);
