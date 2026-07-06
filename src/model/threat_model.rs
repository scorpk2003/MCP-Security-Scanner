use serde_json::Value;


pub struct ThreatModel {
    pub attack_model: AttackModel,
    pub attack_surface: AttackSurface,
    pub protection: AssetProtection,
}

pub enum AssetProtection {
    APIKEYS,
    LOCALFILES,
    USERDATA,
    DATABASECREDENTIALS,
    INTERNALNETWORK,
    HOSTMACHINE,
    AGENTCONTEXT,
    OTHER(String),
}

pub enum AttackModel {
    PROMPTINJECTION,
    DANGEROUSINPUT,
    THIRDPARTY,
    OVERAUTHORIZATION,
    SUPPLYCHAIN,
    OTHER(String),
}

pub enum AttackSurface {
    TOOLNAME(String),
    TOOLDESCRIPTION(String),
    TOOLINPUTSCHEMA(Value),
    TOOLIMPLEMENTATIONCODE(String),
    CONFIGFILE(String),
    FILESYSTEMACCESS(String),
    SHELLCOMMANDEXECUTION,
    NETWORKREQUEST,
    LOGS,
    OTHER(String),
}