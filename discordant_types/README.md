# discordant_types

Type representations for most interactions with Discord.

TODO:
- take a multi-step approach to serialization/deserialization
  - create `raw` classes that represent what Discord sends/expects, and create more ergonomic versions that can be converted `From`/`Into`.
- refactor `ApplicationCommandInteractionDataOption`
  - double check to see if all cases are being handled under latest Discord API version
  - the deserialization is real messy and shouldn't depend on `serde_json`
- refactor `Components`
  - really should be an enum, only letting you set the properties available to each variant.