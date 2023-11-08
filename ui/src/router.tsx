import PrdocPage from "./pages/PrdocPage.tsx";
import { createBrowserRouter } from "react-router-dom";

export const paths = {
  prdocPage: "/",
};

export const basename = "/prdoc";

export const router = createBrowserRouter(
  [
    { path: paths.prdocPage, element: <PrdocPage /> },
  ],
  { basename }
);
