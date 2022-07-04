enum FlowRunSort {
    ID_DESC,
    EXPECTED_START_TIME_ASC,
    EXPECTED_START_TIME_DESC,
    NAME_ASC,
    NAME_DESC,
    NEXT_SCHEDULED_START_TIME_ASC,
    END_TIME_DESC,
}

enum TaskRunSort {
    ID_DESC,
    EXPECTED_START_TIME_ASC,
    EXPECTED_START_TIME_DESC,
    NEXT_SCHEDULED_START_TIME_ASC,
    END_TIME_DESC,
}

enum LogSort {
    TIMESTAMP_ASC,
    TIMESTAMP_DESC,
    LEVEL_ASC,
    LEVEL_DESC,
    FLOW_RUN_ID_ASC,
    FLOW_RUN_ID_DESC,
    TASK_RUN_ID_ASC,
    TASK_RUN_ID_DESC,
}

enum FlowSort {
    CREATED_DESC,
    UPDATED_DESC,
    NAME_ASC,
    NAME_DESC,
}