use cycle_map::CycleMap;
use squire_core::operations::TournOp;
use squire_core::player_registry::PlayerIdentifier;
use squire_core::swiss_pairings::{PlayerId, TournamentError};
use squire_core::tournament::{Tournament, TournamentId, TournamentPreset};

use dashmap::{DashMap, DashSet};
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId, MessageId, RoleId, UserId};
use serenity::prelude::*;

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

// Make these (de)serializable once Tournament becomes so
//#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Debug, Clone)]
pub struct GuildTournament {
    pub(crate) tourn_role: RoleId,
    pub(crate) judge_role: RoleId,
    pub(crate) tourn_admin_role: RoleId,
    pub(crate) pairings_channel: ChannelId,
    pub(crate) matches_category: ChannelId,
    pub(crate) make_vc: bool,
    pub(crate) match_vcs: HashSet<ChannelId>,
    pub(crate) make_tc: bool,
    pub(crate) match_tcs: HashSet<ChannelId>,
    pub(crate) tourn_status: Option<MessageId>,
    pub(crate) players: CycleMap<UserId, PlayerId>,
    pub(crate) match_roles: HashSet<RoleId>,
    pub(crate) match_timers: HashSet<MessageId>,
    pub(crate) standings_messages: Vec<MessageId>,
    pub(crate) tourn: Tournament,
}

impl GuildTournament {
    pub fn new(
        tourn_role: RoleId,
        judge_role: RoleId,
        tourn_admin_role: RoleId,
        pairings_channel: ChannelId,
        matches_category: ChannelId,
        make_vc: bool,
        make_tc: bool,
        preset: TournamentPreset,
        format: String,
        name: String,
    ) -> Self {
        Self {
            tourn_role,
            judge_role,
            tourn_admin_role,
            pairings_channel,
            matches_category,
            make_vc,
            match_vcs: HashSet::new(),
            make_tc,
            match_tcs: HashSet::new(),
            tourn_status: None,
            players: CycleMap::new(),
            match_roles: HashSet::new(),
            match_timers: HashSet::new(),
            standings_messages: Vec::new(),
            tourn: Tournament::from_preset(name, preset, format),
        }
    }

    pub fn get_id(&self) -> TournamentId {
        self.tourn.id.clone()
    }

    pub fn get_player_id(&self, user: &UserId) -> Option<PlayerId> {
        if let Some(id) = self.players.get_right(&user) {
            Some(id.clone())
        } else {
            None
        }
    }

    pub fn get_user_id(&self, user: &PlayerId) -> Option<UserId> {
        if let Some(id) = self.players.get_left(&user) {
            Some(id.clone())
        } else {
            None
        }
    }

    pub fn add_player(&mut self, name: String, user: UserId) -> Result<(), TournamentError> {
        let name_copy = name.clone();
        self.tourn.apply_op(TournOp::RegisterPlayer(name))?;
        let plyr = self
            .tourn
            .get_player(&PlayerIdentifier::Name(name_copy))
            .unwrap()
            .id
            .clone();
        self.players.insert(user, plyr);
        Ok(())
    }
}

impl Hash for GuildTournament {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let _ = &self.tourn.hash(state);
    }
}