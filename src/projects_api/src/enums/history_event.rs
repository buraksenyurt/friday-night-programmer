use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum HistoryEvent {
    CreatedNewAssignment,
    AssignmentStatusChanged,
    CriteriaSetCreated,
    CriterionAddedToSet,
    ProjectCreated,
    TeamCreated,
    MemberAddedToTeam,
    ScoresUpdated,
    TeamDeleted,
    MemberMoved,
    CriterionDeleted,
}

impl HistoryEvent {
    fn as_str(&self) -> &'static str {
        match self {
            HistoryEvent::CreatedNewAssignment => "CreatedNewAssignment",
            HistoryEvent::AssignmentStatusChanged => "AssignmentStatusChanged",
            HistoryEvent::CriteriaSetCreated => "CriteriaSetCreated",
            HistoryEvent::CriterionAddedToSet => "CriterionAddedToSet",
            HistoryEvent::ProjectCreated => "ProjectCreated",
            HistoryEvent::TeamCreated => "TeamCreated",
            HistoryEvent::MemberAddedToTeam => "MemberAddedToTeam",
            HistoryEvent::ScoresUpdated => "ScoresUpdated",
            HistoryEvent::TeamDeleted => "TeamDeleted",
            HistoryEvent::MemberMoved => "MemberMoved",
            HistoryEvent::CriterionDeleted => "CriterionDeleted",
        }
    }
}

impl Display for HistoryEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
