use anyhow::Result;
use openapi::models::TasksGetRequest;

use domain::task::TaskId;
use usecase::task::ListTaskQuery;

pub struct TasksGetRequestWrapper(TasksGetRequest);

impl TryFrom<TasksGetRequest> for TasksGetRequestWrapper {
    type Error = anyhow::Error;

    fn try_from(value: TasksGetRequest) -> Result<Self> {
        Ok(Self(value))
    }
}

impl TryFrom<TasksGetRequestWrapper> for ListTaskQuery {
    type Error = anyhow::Error;

    fn try_from(wrapper: TasksGetRequestWrapper) -> Result<Self> {
        fn by_id_list(request: TasksGetRequest) -> Result<ListTaskQuery> {
            let id_list = request.id_list.clone().unwrap_or_default();
            let id_list = id_list
                .into_iter()
                .map(|s| TaskId::try_from(s.clone()).expect(&format!("Invalid task id: {}", s)))
                .collect();
            Ok(ListTaskQuery::ByIdList(id_list))
        }

        fn by_assignee(request: TasksGetRequest) -> Result<ListTaskQuery> {
            let assignee = request.assignee.expect("Invalid assignee");
            Ok(ListTaskQuery::ByAssignee(assignee))
        }

        fn by_status(request: TasksGetRequest) -> Result<ListTaskQuery> {
            let status = request.status.expect("Invalid task status");
            Ok(ListTaskQuery::ByStatus(
                status.to_string().try_into().expect("Invalid task status"),
            ))
        }

        let request = wrapper.0;
        let query_type = request.query_type.clone();

        let query = match query_type.as_str() {
            "IdList" => by_id_list(request.clone()),
            "Assignee" => by_assignee(request.clone()),
            "Status" => by_status(request.clone()),
            _ => anyhow::bail!("Invalid query type: {}", query_type),
        };

        query
    }
}
