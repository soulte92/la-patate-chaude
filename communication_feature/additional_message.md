### Les types additionnels:

| Nom du type               | Description du type                                                                                                                                                   |
|---------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `SubscribeError`          | `enum { AlreadyRegistered, InvalidName }`                                                                                                                             |
| `PublicPlayer`            | `name: String`<br/>`stream_id: String `<br/>`score: i32 `<br/>`steps: u32 `<br/>`is_active: bool`<br/>`total_used_time: f64 `                                         |
| `ChallengeAnswer`         | `enum { ChallengeName(ChallengeOutput) }`                                                                                                                             |
| `ChallengeResult`         | `name:  ChallengeAnswer`<br/>`next_target: String`                                                                                                                    |
| `ChallengeValue`          | `enum {`<br/>`  Unreachable,`<br/>`  Timeout,`<br/>`  BadResult { used_time: f64, next_target: String },`<br/>`  Ok { used_time: f64, next_target: String }`<br/>` }` |
| `ReportedChallengeResult` | `name: String,`<br/>`value: ChallengeValue`                                                                                                                           |
| `PublicLeaderBoard`       | `.0: Vec<PublicPlayer>`                                                                                                                                               |