use core::types::user::*;
use core::types::error::*;
use rocket::{Data, Outcome, Request};
use rocket::http::Status;
use rocket::data::{self, FromData};
use rocket_contrib::Json;

use rocket::response::{Responder, Response};
use http::errors::io::*;
use chrono::Local;
use rocket::response;
use rocket;
use serde_json;
use http::api::ApiResult;

impl FromData for RegisterUserInput {
    type Error = RegisterUserError;
    fn from_data(req: &Request, data: Data) -> data::Outcome<RegisterUserInput, Self::Error> {
        let unwrapped_json = Json::<RegisterUserInput>::from_data(&req, data);

        if let Outcome::Failure(error) = unwrapped_json {
            let (_, serde_error) = error;

            return Outcome::Failure((
                Status::from_code(422).unwrap(),
                RegisterUserError::InvalidInput(serde_error.to_string()),
            ));
        }

        let json = unwrapped_json.unwrap().0;
        Outcome::Success(json)
    }
}

impl<'r> Responder<'r> for ApiResult<CurrentUserOutput, CurrentUserError> {
    fn respond_to(self, req: &Request) -> Result<rocket::Response<'r>, Status> {
        let date = Local::now();

        let mut build = Response::build();

        match self.0 {
            Ok(output) => {
                build
                    .merge(response::content::Json(serde_json::to_string(&output)).respond_to(req)?);
                build.status(Status::Ok).ok()
            }
            Err(err) => {
                let err_response = match err {
                    CurrentUserError::RepoError(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("repo_error"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("RepoError"),
                        },
                    },
                    CurrentUserError::InvalidInput(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("invalid_input"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("IncompleteOrInvalidParameterException"),
                        },
                    },
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}

impl FromData for LoginUserInput {
    type Error = LoginUserError;
    fn from_data(req: &Request, data: Data) -> data::Outcome<LoginUserInput, Self::Error> {
        let unwrapped_json = Json::<LoginUserInput>::from_data(&req, data);

        if let Outcome::Failure(error) = unwrapped_json {
            let (_, serde_error) = error;

            return Outcome::Failure((
                Status::from_code(422).unwrap(),
                LoginUserError::InvalidInput(serde_error.to_string()),
            ));
        }

        let json = unwrapped_json.unwrap().0;
        Outcome::Success(json)
    }
}


impl<'r> Responder<'r> for ApiResult<RegisterUserOutput, RegisterUserError> {
    fn respond_to(self, req: &Request) -> Result<rocket::Response<'r>, Status> {
        let date = Local::now();

        let mut build = Response::build();

        match self.0 {
            Ok(output) => {
                build
                    .merge(response::content::Json(serde_json::to_string(&output)).respond_to(req)?);
                build.status(Status::Ok).ok()
            }
            Err(err) => {
                let err_response = match err {
                    RegisterUserError::RepoError(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("repo_error"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("RepoError"),
                        },
                    },
                    RegisterUserError::InvalidInput(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("invalid_input"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("IncompleteOrInvalidParameterException"),
                        },
                    },
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}

impl<'r> Responder<'r> for ApiResult<LoginUserOutput, LoginUserError> {
    fn respond_to(self, req: &Request) -> Result<rocket::Response<'r>, Status> {
        let date = Local::now();

        let mut build = Response::build();

        match self.0 {
            Ok(output) => {
                build
                    .merge(response::content::Json(serde_json::to_string(&output)).respond_to(req)?);
                build.status(Status::Ok).ok()
            }
            Err(err) => {
                let err_response = match err {
                    LoginUserError::RepoError(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("repo_error"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("RepoError"),
                        },
                    },
                    LoginUserError::InvalidInput(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("invalid_input"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("IncompleteOrInvalidParameterException"),
                        },
                    },
                    LoginUserError::InvalidCredentials => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from("Invalid credentials"),
                            message_shortcode: String::from("invalid_credentials"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("IncompleteOrInvalidParameterException"),
                        },
                    },
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}

impl FromData for UpdateUserInput {
    type Error = UpdateUserError;
    fn from_data(req: &Request, data: Data) -> data::Outcome<UpdateUserInput, Self::Error> {
        let unwrapped_json = Json::<UpdateUserInput>::from_data(&req, data);

        if let Outcome::Failure(error) = unwrapped_json {
            let (_, serde_error) = error;

            return Outcome::Failure((
                Status::from_code(422).unwrap(),
                UpdateUserError::InvalidInput(serde_error.to_string()),
            ));
        }

        let json = unwrapped_json.unwrap().0;
        Outcome::Success(json)
    }
}

impl<'r> Responder<'r> for ApiResult<UpdateUserOutput, UpdateUserError> {
    fn respond_to(self, req: &Request) -> Result<rocket::Response<'r>, Status> {
        let date = Local::now();

        let mut build = Response::build();

        match self.0 {
            Ok(output) => {
                build
                    .merge(response::content::Json(serde_json::to_string(&output)).respond_to(req)?);
                build.status(Status::Ok).ok()
            }
            Err(err) => {
                let err_response = match err {
                    UpdateUserError::RepoError(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("repo_error"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("RepoError"),
                        },
                    },
                    UpdateUserError::InvalidInput(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("invalid_input"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("IncompleteOrInvalidParameterException"),
                        },
                    },
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}
