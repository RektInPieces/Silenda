import React from "react";
import Button from "../components/Button";

export default function Home() {
  return (
    <>
      <header>
        <h1>Silenda</h1>
        <p>A card game builder</p>
      </header>
      <p>
        <Button>Build a Game</Button> <Button>Play a Game</Button>
      </p>
    </>
  );
}
