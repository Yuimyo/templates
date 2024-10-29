'use client'

import Button from "./Button";
import css from "./Square.module.css";

export default function Square({}) {
    return <>
        <div className={css.content}>
            <Button text="ここを押して" on_click={() => {
                console.log("AAA");
            }}/>
        </div>
    </>
}