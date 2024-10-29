import css from "./Button.module.css";

type Props = {
    text?: string;
    on_click: () => void;
};

export default function Button(props: Props) {
    return <>
        <button className={css.content} 
            onClick={props.on_click}>
            {props.text}
        </button>
    </>
}