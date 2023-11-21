use validator::Validate;

use super::request_content::UpdateMinimalAuthorRequestContent;
use super::response_content_failure::UpdateMinimalAuthorContentFailure;
use super::response_content_failure::UpdateMinimalAuthorContentFailure::*;
use super::response_content_success::UpdateMinimalAuthorContentSuccess;

pub async fn http_handler(
    (UpdateMinimalAuthorRequestContent {
        updated_minimal_author_data,
        author_service,
        auth_author_future,
    },): (UpdateMinimalAuthorRequestContent,),
) -> Result<UpdateMinimalAuthorContentSuccess, UpdateMinimalAuthorContentFailure> {
    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.base.blocked == 1 {
        return Err(EditingForbidden);
    }

    let base_minimal_author = updated_minimal_author_data.map_err(|e| ValidationError {
        reason: e.to_string(),
    })?;

    base_minimal_author
        .validate()
        .map_err(|e| ValidationError {
            reason: e.to_string(),
        })?;

    let mut base_minimal_author = base_minimal_author;
    base_minimal_author.slug = format!(
        "{slug}#{id}",
        slug = base_minimal_author.slug,
        id = author.id,
    );

    author_service
        .update_minimal_author_by_id(&From::from(base_minimal_author), &author.id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(().into())
}