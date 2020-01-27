import { createReducer } from "@reduxjs/toolkit";
import { %file_name%Success } from "./%file_name%.actions";

export interface %c_file_name%State {
    isLoading: boolean;
}

export const %file_name%InitState: %c_file_name%State = {
    isLoading: true,
}

export const %file_name%Reducer = createReducer<%c_file_name%State>(%file_name%InitState, (builder) => {
    builder
        .addCase(%file_name%Success, () => {
        return {
            isLoading: false
        }
    })
});