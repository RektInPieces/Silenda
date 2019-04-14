import React from "react";
import Button from "../components/Button";
import { withRouter, RouteComponentProps } from "react-router";

function Home({ history }: RouteComponentProps<{}>) {
  return (
    <>
      <header>
        <h1>Silenda</h1>
        <p>A card game builder</p>
      </header>
      <p>
        <button onClick={() => history.push("/build")}>Build a Game</button>{" "}
        <Button>Play a Game</Button>
      </p>
    </>
  );
}

export default withRouter(Home);
