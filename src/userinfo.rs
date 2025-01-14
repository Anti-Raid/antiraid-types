use extract_map::ExtractMap;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserInfo {
    pub discord_permissions: serenity::all::Permissions,
    pub kittycat_staff_permissions: kittycat::perms::StaffPermissions,
    pub kittycat_resolved_permissions: Vec<kittycat::perms::Permission>,
    pub guild_owner_id: serenity::all::UserId,
    pub guild_roles: ExtractMap<serenity::all::RoleId, serenity::all::Role>,
    pub member_roles: Vec<serenity::all::RoleId>,
}

impl std::fmt::Debug for UserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserInfo")
            .field("discord_permissions", &self.discord_permissions)
            .field(
                "kittycat_resolved_permissions",
                &self.kittycat_resolved_permissions,
            )
            .field("guild_owner_id", &self.guild_owner_id)
            .field("guild_roles", &self.guild_roles)
            .field("member_roles", &self.member_roles)
            .finish()
    }
}
