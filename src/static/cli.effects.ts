import { takeLatest } from "redux-saga/effects";
import { %file_name%Init } from "./%file_name%.actions";

function* increaseCount() {
    try {
    } catch (error) {
    }
}

export const counterEffects = [
    takeLatest(%file_name%Init, increaseCount),
];
