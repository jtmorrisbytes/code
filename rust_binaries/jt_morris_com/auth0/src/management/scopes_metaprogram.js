const scopes = ["client_grants",
"users",
"users_app_metadata",
"user_custom_blocks",
"user_custom_tickets",
"clients",
"client_keys",
"connections",
"resource_servers",
"device_credentials",
"rules",
"rules_configs",
"hooks",
"actions",
"email_provider",
"tokens",
"stats",
"insights",
"tenant_settings",
"logs",
"logs_users",
"shields",
"anomaly_blocks",
"triggers",
"grants",
"guardian_factors",
"guardian_enrollments",
"guardian_enrollment_tickets",
"user_idp_tokens",
"passwords_checking_job",
"custom_domains",
"email_templates",
"mfa_policies",
"roles",
"prompts",
"branding",
"log_streams",
"signing_keys",
"limits",
"role_members",
"entitlements",
"attack_protection",
"organizations_summary",
"authentication_methods",
"organizations",
"organization_members",
"organization_connections",
"organization_member_roles",
"organization_invitations",
"phone_providers",
"phone_templates",
"encryption_keys",
"sessions",
"refresh_tokens",
"client_credentials"]
const actions = ["create","read","update","delete"];
const fs = require('fs')
let file = fs.openSync(__dirname + "/scopes.rs","w");


function scopeStringNameIdent(action,scope) {
    return`SCOPE_${action.toUpperCase()}_${scope.toUpperCase()}`
}
function scopeStructNameIdent(action,scope) {
    return (action.slice(0,1).toUpperCase() + action.slice(1)) + (scope.split("_").map((s)=>{ return (s.slice(0,1).toUpperCase() + s.slice(1))}).join(""))
}



for (scope of scopes) {
    for (action of actions) {
    fs.writeSync(file,`pub const ${scopeStringNameIdent(action,scope)}: &'static str = "${action}:${scope}";\n`)
    }    
}
let tokens =`
#[cfg_attr(any(feature="debug",debug_asertions),derive(Debug))]
#[derive(serde::Serialize,serde::Deserialize,Clone,PartialEq,Eq,Hash)]
pub enum Scope {
`
for (scope of scopes) {

    for (action of actions) {

        let structName = scopeStructNameIdent(action,scope)

tokens += 
`\t#[serde(alias="${action}:${scope}")]
\t${structName},\n`
    }
}
tokens +="}"
fs.writeSync(file,tokens)

// write impl display for struct

let match_arms = ""
for (scope of scopes) {

    for (action of actions) {

        let structName = scopeStructNameIdent(action,scope)

match_arms += 
`Self::${structName} => write!(f,"{}",${scopeStringNameIdent(action,scope)}),\n`
    }
}


tokens = `impl std::fmt::Display for Scope{
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
         ${match_arms}
        }
    }
}
`;

// write impl std::str::FromStr for enun

match_arms = ""
for (scope of scopes) {

    for (action of actions) {


match_arms += 
`      ${scopeStringNameIdent(action,scope)}  => Ok(Self::${scopeStructNameIdent(action,scope)}),\n`
    }
}

tokens = 
`impl std::str::FromStr for Scope {
    type Err = ();
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        match s {
            ${match_arms}
            _ => Err(())
        }

    }
}`
// write impl AsRef<Str> for Scope
fs.writeSync(file,tokens)
match_arms = ""
for (scope of scopes) {

    for (action of actions) {


match_arms += 
`      Self::${scopeStructNameIdent(action,scope)} => ${scopeStringNameIdent(action,scope)},\n`
    }
}

tokens = 
`impl AsRef<str> for Scope {
    fn as_ref(&self) -> &'static str {
        match self {
            ${match_arms}
        }

    }
}`
fs.writeSync(file,tokens)



fs.closeSync(file)
