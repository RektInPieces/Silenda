import React from "react";
import styles from "./App.module.scss";
import Home from "./pages/Home";
import { BrowserRouter, Switch, Route } from "react-router-dom";

export default function App() {
  return (
    <div className={styles.App}>
      <BrowserRouter>
        <Switch>
          <Route path="/" exact component={Home} />
        </Switch>
      </BrowserRouter>
    </div>
  );
}
