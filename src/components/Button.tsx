import React from "react";
import styles from "./Button.module.scss";

export default function Button({ children }: { children: React.ReactChild }) {
  return <button className={styles.Button}>{children}</button>;
}
