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
}

impl Display for HistoryEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoryEvent::CreatedNewAssignment => {
                write!(f, "CreatedNewAssignment")
            }
            HistoryEvent::AssignmentStatusChanged => {
                write!(f, "AssignmentStatusChanged")
            }
            HistoryEvent::CriteriaSetCreated => {
                write!(f, "CriteriaSetCreated")
            }
            HistoryEvent::CriterionAddedToSet => {
                write!(f, "CriterionAddedToSet")
            }
            HistoryEvent::ProjectCreated => {
                write!(f, "ProjectCreated")
            }
            HistoryEvent::TeamCreated => {
                write!(f, "TeamCreated")
            }
            HistoryEvent::MemberAddedToTeam => {
                write!(f, "MemberAddedToTeam")
            }
            HistoryEvent::ScoresUpdated => {
                write!(f, "ScoresUpdated")
            }
            HistoryEvent::TeamDeleted => {
                write!(f, "TeamDeleted")
            }
            HistoryEvent::MemberMoved => {
                write!(f, "MemberMoved")
            }
        }
    }
}
