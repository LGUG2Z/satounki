pub use delete::*;
pub use get_id::*;
pub use get_list::*;
pub use patch_id::*;
pub use patch_id_body::*;
pub use post::*;
pub use post_id::*;
pub use put::*;
pub use put_id::*;
pub use put_id_body::*;

mod delete;
mod get_id;
mod get_list;
mod patch_id;
mod patch_id_body;
mod post;
mod post_id;
mod put;
mod put_id;
mod put_id_body;

pub const TS_TEMPLATE: &str = r#"{{ comment }}

import { IRestResponse, RestClient } from "typed-rest-client/RestClient";
import * as satounki from "./types.generated";

export class Api {
  private apiToken: string;
  private baseUrl: string;
  private userAgent: string;
  client: RestClient;

  constructor(apiToken: string, baseUrl: string, userAgent: string) {
    let restClient = new RestClient(userAgent, baseUrl, [], {
      headers: {
        "Authorization": `Bearer ${apiToken}`,
      },
    });

    this.apiToken = apiToken;
    this.baseUrl = baseUrl;
    this.userAgent = userAgent;
    this.client = restClient;
  }
  
  {{ generated }}
}
"#;
