use crate::paginate_all;
use crate::types::{Column, Row, Table, TableReference};
use crate::{ClientTablesError, Error, Limiter, RawClient, ResponseValue, RichRow, RichRowList, RowUpdateResultCorrect, RowsUpsertResultCorrect, TableId, types};
use error_handling::handle;
use std::collections::HashMap;

pub struct Client {
    pub raw: RawClient,
    pub limiter: Limiter,
}

impl Client {
    pub const BASE_URL: &'static str = RawClient::BASE_URL;

    pub fn new(baseurl: &str) -> Self {
        let raw = RawClient::new(baseurl);
        let limiter = Limiter::default();
        Self {
            raw,
            limiter,
        }
    }

    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        let raw = RawClient::new_with_client(baseurl, client);
        let limiter = Limiter::default();
        Self {
            raw,
            limiter,
        }
    }

    pub fn new_with_key(api_key: &str) -> reqwest::Result<Self> {
        let raw = RawClient::new_with_key(api_key)?;
        let limiter = Limiter::default();

        Ok(Self {
            raw,
            limiter,
        })
    }

    pub async fn list_categories<'a>(&'a self) -> Result<ResponseValue<types::DocCategoryList>, Error<types::ListCategoriesResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.list_categories().await
    }

    pub async fn list_docs<'a>(&'a self, folder_id: Option<&'a str>, in_gallery: Option<bool>, is_owner: Option<bool>, is_published: Option<bool>, is_starred: Option<bool>, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>, query: Option<&'a str>, source_doc: Option<&'a str>, workspace_id: Option<&'a str>) -> Result<ResponseValue<types::DocList>, Error<types::ListDocsResponse>> {
        self.limiter.list_docs.until_ready().await;
        self.raw
            .list_docs(folder_id, in_gallery, is_owner, is_published, is_starred, limit, page_token, query, source_doc, workspace_id)
            .await
    }

    pub async fn create_doc<'a>(&'a self, body: &'a types::DocCreate) -> Result<ResponseValue<types::DocumentCreationResult>, Error<types::CreateDocResponse>> {
        self.limiter.doc_content_write.until_ready().await;
        self.raw.create_doc(body).await
    }

    pub async fn get_doc<'a>(&'a self, doc_id: &'a str) -> Result<ResponseValue<types::Doc>, Error<types::GetDocResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_doc(doc_id).await
    }

    pub async fn delete_doc<'a>(&'a self, doc_id: &'a str) -> Result<ResponseValue<types::DocDelete>, Error<types::DeleteDocResponse>> {
        self.limiter.doc_content_write.until_ready().await;
        self.raw.delete_doc(doc_id).await
    }

    pub async fn update_doc<'a>(&'a self, doc_id: &'a str, body: &'a types::DocUpdate) -> Result<ResponseValue<types::DocUpdateResult>, Error<types::UpdateDocResponse>> {
        self.limiter.doc_content_write.until_ready().await;
        self.raw.update_doc(doc_id, body).await
    }

    pub async fn get_sharing_metadata<'a>(&'a self, doc_id: &'a str) -> Result<ResponseValue<types::AclMetadata>, Error<types::GetSharingMetadataResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_sharing_metadata(doc_id).await
    }

    pub async fn get_permissions<'a>(&'a self, doc_id: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>) -> Result<ResponseValue<types::Acl>, Error<types::GetPermissionsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_permissions(doc_id, limit, page_token).await
    }

    pub async fn add_permission<'a>(&'a self, doc_id: &'a str, body: &'a types::AddPermissionRequest) -> Result<ResponseValue<types::AddPermissionResult>, Error<types::AddPermissionResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.add_permission(doc_id, body).await
    }

    pub async fn delete_permission<'a>(&'a self, doc_id: &'a str, permission_id: &'a str) -> Result<ResponseValue<types::DeletePermissionResult>, Error<types::DeletePermissionResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.delete_permission(doc_id, permission_id).await
    }

    pub async fn search_principals<'a>(&'a self, doc_id: &'a str, query: Option<&'a str>) -> Result<ResponseValue<types::SearchPrincipalsResponse>, Error<types::SearchPrincipalsResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.search_principals(doc_id, query).await
    }

    pub async fn get_acl_settings<'a>(&'a self, doc_id: &'a str) -> Result<ResponseValue<types::AclSettings>, Error<types::GetAclSettingsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_acl_settings(doc_id).await
    }

    pub async fn update_acl_settings<'a>(&'a self, doc_id: &'a str, body: &'a types::UpdateAclSettingsRequest) -> Result<ResponseValue<types::AclSettings>, Error<types::UpdateAclSettingsResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.update_acl_settings(doc_id, body).await
    }

    pub async fn publish_doc<'a>(&'a self, doc_id: &'a str, body: &'a types::DocPublish) -> Result<ResponseValue<types::PublishResult>, Error<types::PublishDocResponse>> {
        self.limiter.doc_content_write.until_ready().await;
        self.raw.publish_doc(doc_id, body).await
    }

    pub async fn unpublish_doc<'a>(&'a self, doc_id: &'a str) -> Result<ResponseValue<types::UnpublishResult>, Error<types::UnpublishDocResponse>> {
        self.limiter.doc_content_write.until_ready().await;
        self.raw.unpublish_doc(doc_id).await
    }

    pub async fn list_pages<'a>(&'a self, doc_id: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>) -> Result<ResponseValue<types::PageList>, Error<types::ListPagesResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.list_pages(doc_id, limit, page_token).await
    }

    pub async fn create_page<'a>(&'a self, doc_id: &'a str, body: &'a types::PageCreate) -> Result<ResponseValue<types::PageCreateResult>, Error<types::CreatePageResponse>> {
        self.limiter.doc_content_write.until_ready().await;
        self.raw.create_page(doc_id, body).await
    }

    pub async fn get_page<'a>(&'a self, doc_id: &'a str, page_id_or_name: &'a str) -> Result<ResponseValue<types::Page>, Error<types::GetPageResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_page(doc_id, page_id_or_name).await
    }

    pub async fn update_page<'a>(&'a self, doc_id: &'a str, page_id_or_name: &'a str, body: &'a types::PageUpdate) -> Result<ResponseValue<types::PageUpdateResult>, Error<types::UpdatePageResponse>> {
        self.limiter.doc_content_write.until_ready().await;
        self.raw.update_page(doc_id, page_id_or_name, body).await
    }

    pub async fn delete_page<'a>(&'a self, doc_id: &'a str, page_id_or_name: &'a str) -> Result<ResponseValue<types::PageDeleteResult>, Error<types::DeletePageResponse>> {
        self.limiter.doc_content_write.until_ready().await;
        self.raw.delete_page(doc_id, page_id_or_name).await
    }

    pub async fn begin_page_content_export<'a>(&'a self, doc_id: &'a str, page_id_or_name: &'a str, body: &'a types::BeginPageContentExportRequest) -> Result<ResponseValue<types::BeginPageContentExportResponse>, Error<types::BeginPageContentExportResponse>> {
        self.limiter.doc_content_write.until_ready().await;
        self.raw
            .begin_page_content_export(doc_id, page_id_or_name, body)
            .await
    }

    pub async fn get_page_content_export_status<'a>(&'a self, doc_id: &'a str, page_id_or_name: &'a str, request_id: &'a str) -> Result<ResponseValue<types::PageContentExportStatusResponse>, Error<types::GetPageContentExportStatusResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .get_page_content_export_status(doc_id, page_id_or_name, request_id)
            .await
    }

    pub async fn list_tables<'a>(&'a self, doc_id: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>, sort_by: Option<types::SortBy>, table_types: Option<&'a ::std::vec::Vec<types::TableTypeEnum>>) -> Result<ResponseValue<types::TableList>, Error<types::ListTablesResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_tables(doc_id, limit, page_token, sort_by, table_types)
            .await
    }

    pub async fn get_table<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, use_updated_table_layouts: Option<bool>) -> Result<ResponseValue<types::Table>, Error<types::GetTableResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .get_table(doc_id, table_id_or_name, use_updated_table_layouts)
            .await
    }

    pub async fn list_columns<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>, visible_only: Option<bool>) -> Result<ResponseValue<types::ColumnList>, Error<types::ListColumnsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_columns(doc_id, table_id_or_name, limit, page_token, visible_only)
            .await
    }

    pub async fn list_rows<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>, query: Option<&'a str>, sort_by: Option<types::RowsSortBy>, sync_token: Option<&'a str>, use_column_names: Option<bool>, value_format: Option<types::ValueFormat>, visible_only: Option<bool>) -> Result<ResponseValue<types::RowList>, Error<types::ListRowsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_rows(doc_id, table_id_or_name, limit, page_token, query, sort_by, sync_token, use_column_names, value_format, visible_only)
            .await
    }

    pub async fn upsert_rows<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, disable_parsing: Option<bool>, body: &'a types::RowsUpsert) -> Result<ResponseValue<types::RowsUpsertResult>, Error<types::UpsertRowsResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .upsert_rows(doc_id, table_id_or_name, disable_parsing, body)
            .await
    }

    pub async fn delete_rows<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, body: &'a types::RowsDelete) -> Result<ResponseValue<types::RowsDeleteResult>, Error<types::DeleteRowsResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.delete_rows(doc_id, table_id_or_name, body).await
    }

    pub async fn get_row<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, row_id_or_name: &'a str, use_column_names: Option<bool>, value_format: Option<types::ValueFormat>) -> Result<ResponseValue<types::RowDetail>, Error<types::GetRowResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .get_row(doc_id, table_id_or_name, row_id_or_name, use_column_names, value_format)
            .await
    }

    pub async fn update_row<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, row_id_or_name: &'a str, disable_parsing: Option<bool>, body: &'a types::RowUpdate) -> Result<ResponseValue<types::RowUpdateResult>, Error<types::UpdateRowResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .update_row(doc_id, table_id_or_name, row_id_or_name, disable_parsing, body)
            .await
    }

    pub async fn delete_row<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, row_id_or_name: &'a str) -> Result<ResponseValue<types::RowDeleteResult>, Error<types::DeleteRowResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .delete_row(doc_id, table_id_or_name, row_id_or_name)
            .await
    }

    pub async fn push_button<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, row_id_or_name: &'a str, column_id_or_name: &'a str) -> Result<ResponseValue<types::PushButtonResult>, Error<types::PushButtonResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .push_button(doc_id, table_id_or_name, row_id_or_name, column_id_or_name)
            .await
    }

    pub async fn get_column<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, column_id_or_name: &'a str) -> Result<ResponseValue<types::ColumnDetail>, Error<types::GetColumnResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .get_column(doc_id, table_id_or_name, column_id_or_name)
            .await
    }

    pub async fn list_formulas<'a>(&'a self, doc_id: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>, sort_by: Option<types::SortBy>) -> Result<ResponseValue<types::FormulaList>, Error<types::ListFormulasResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_formulas(doc_id, limit, page_token, sort_by)
            .await
    }

    pub async fn get_formula<'a>(&'a self, doc_id: &'a str, formula_id_or_name: &'a str) -> Result<ResponseValue<types::Formula>, Error<types::GetFormulaResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_formula(doc_id, formula_id_or_name).await
    }

    pub async fn list_controls<'a>(&'a self, doc_id: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>, sort_by: Option<types::SortBy>) -> Result<ResponseValue<types::ControlList>, Error<types::ListControlsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_controls(doc_id, limit, page_token, sort_by)
            .await
    }

    pub async fn get_control<'a>(&'a self, doc_id: &'a str, control_id_or_name: &'a str) -> Result<ResponseValue<types::Control>, Error<types::GetControlResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_control(doc_id, control_id_or_name).await
    }

    pub async fn list_custom_doc_domains<'a>(&'a self, doc_id: &'a str) -> Result<ResponseValue<types::CustomDocDomainList>, Error<types::ListCustomDocDomainsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.list_custom_doc_domains(doc_id).await
    }

    pub async fn add_custom_doc_domain<'a>(&'a self, doc_id: &'a str, body: &'a types::AddCustomDocDomainRequest) -> Result<ResponseValue<types::AddCustomDocDomainResponse>, Error<types::AddCustomDocDomainResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.add_custom_doc_domain(doc_id, body).await
    }

    pub async fn delete_custom_doc_domain<'a>(&'a self, doc_id: &'a str, custom_doc_domain: &'a str) -> Result<ResponseValue<types::DeleteCustomDocDomainResponse>, Error<types::DeleteCustomDocDomainResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .delete_custom_doc_domain(doc_id, custom_doc_domain)
            .await
    }

    pub async fn update_custom_doc_domain<'a>(&'a self, doc_id: &'a str, custom_doc_domain: &'a str, body: &'a types::UpdateCustomDocDomainRequest) -> Result<ResponseValue<types::UpdateCustomDocDomainResponse>, Error<types::UpdateCustomDocDomainResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .update_custom_doc_domain(doc_id, custom_doc_domain, body)
            .await
    }

    pub async fn get_custom_doc_domain_provider<'a>(&'a self, custom_doc_domain: &'a str) -> Result<ResponseValue<types::CustomDocDomainProviderResponse>, Error<types::GetCustomDocDomainProviderResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .get_custom_doc_domain_provider(custom_doc_domain)
            .await
    }

    pub async fn get_folder<'a>(&'a self, folder_id: &'a str) -> Result<ResponseValue<types::Folder>, Error<types::GetFolderResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_folder(folder_id).await
    }

    pub async fn whoami<'a>(&'a self) -> Result<ResponseValue<types::User>, Error<types::WhoamiResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.whoami().await
    }

    pub async fn resolve_browser_link<'a>(&'a self, degrade_gracefully: Option<bool>, url: &'a str) -> Result<ResponseValue<types::ApiLink>, Error<types::ResolveBrowserLinkResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.resolve_browser_link(degrade_gracefully, url).await
    }

    pub async fn get_mutation_status<'a>(&'a self, request_id: &'a str) -> Result<ResponseValue<types::MutationStatus>, Error<types::GetMutationStatusResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_mutation_status(request_id).await
    }

    pub async fn trigger_webhook_automation<'a>(&'a self, doc_id: &'a str, rule_id: &'a str, body: &'a types::WebhookTriggerPayload) -> Result<ResponseValue<types::WebhookTriggerResult>, Error<types::TriggerWebhookAutomationResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .trigger_webhook_automation(doc_id, rule_id, body)
            .await
    }

    pub async fn list_doc_analytics<'a>(&'a self, direction: Option<types::SortDirection>, doc_ids: Option<&'a ::std::vec::Vec<::std::string::String>>, is_published: Option<bool>, limit: Option<::std::num::NonZeroU64>, order_by: Option<types::DocAnalyticsOrderBy>, page_token: Option<&'a str>, query: Option<&'a str>, scale: Option<types::AnalyticsScale>, since_date: Option<&'a ::chrono::naive::NaiveDate>, until_date: Option<&'a ::chrono::naive::NaiveDate>, workspace_id: Option<&'a str>) -> Result<ResponseValue<types::DocAnalyticsCollection>, Error<types::ListDocAnalyticsResponse>> {
        self.limiter.analytics.until_ready().await;
        self.raw
            .list_doc_analytics(direction, doc_ids, is_published, limit, order_by, page_token, query, scale, since_date, until_date, workspace_id)
            .await
    }

    pub async fn list_page_analytics<'a>(&'a self, doc_id: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>, since_date: Option<&'a ::chrono::naive::NaiveDate>, until_date: Option<&'a ::chrono::naive::NaiveDate>) -> Result<ResponseValue<types::PageAnalyticsCollection>, Error<types::ListPageAnalyticsResponse>> {
        self.limiter.analytics.until_ready().await;
        self.raw
            .list_page_analytics(doc_id, limit, page_token, since_date, until_date)
            .await
    }

    pub async fn list_doc_analytics_summary<'a>(&'a self, is_published: Option<bool>, since_date: Option<&'a ::chrono::naive::NaiveDate>, until_date: Option<&'a ::chrono::naive::NaiveDate>, workspace_id: Option<&'a str>) -> Result<ResponseValue<types::DocAnalyticsSummary>, Error<types::ListDocAnalyticsSummaryResponse>> {
        self.limiter.analytics.until_ready().await;
        self.raw
            .list_doc_analytics_summary(is_published, since_date, until_date, workspace_id)
            .await
    }

    pub async fn list_pack_analytics<'a>(&'a self, direction: Option<types::SortDirection>, is_published: Option<bool>, limit: Option<::std::num::NonZeroU64>, order_by: Option<types::PackAnalyticsOrderBy>, pack_ids: Option<&'a ::std::vec::Vec<i64>>, page_token: Option<&'a str>, query: Option<&'a str>, scale: Option<types::AnalyticsScale>, since_date: Option<&'a ::chrono::naive::NaiveDate>, until_date: Option<&'a ::chrono::naive::NaiveDate>, workspace_id: Option<&'a str>) -> Result<ResponseValue<types::PackAnalyticsCollection>, Error<types::ListPackAnalyticsResponse>> {
        self.limiter.analytics.until_ready().await;
        self.raw
            .list_pack_analytics(direction, is_published, limit, order_by, pack_ids, page_token, query, scale, since_date, until_date, workspace_id)
            .await
    }

    pub async fn list_pack_analytics_summary<'a>(&'a self, is_published: Option<bool>, pack_ids: Option<&'a ::std::vec::Vec<i64>>, since_date: Option<&'a ::chrono::naive::NaiveDate>, until_date: Option<&'a ::chrono::naive::NaiveDate>, workspace_id: Option<&'a str>) -> Result<ResponseValue<types::PackAnalyticsSummary>, Error<types::ListPackAnalyticsSummaryResponse>> {
        self.limiter.analytics.until_ready().await;
        self.raw
            .list_pack_analytics_summary(is_published, pack_ids, since_date, until_date, workspace_id)
            .await
    }

    pub async fn list_pack_formula_analytics<'a>(&'a self, pack_id: ::std::num::NonZeroU64, direction: Option<types::SortDirection>, limit: Option<::std::num::NonZeroU64>, order_by: Option<types::PackFormulaAnalyticsOrderBy>, pack_formula_names: Option<&'a ::std::vec::Vec<::std::string::String>>, pack_formula_types: Option<&'a ::std::vec::Vec<types::PackFormulaType>>, page_token: Option<&'a str>, scale: Option<types::AnalyticsScale>, since_date: Option<&'a ::chrono::naive::NaiveDate>, until_date: Option<&'a ::chrono::naive::NaiveDate>) -> Result<ResponseValue<types::PackFormulaAnalyticsCollection>, Error<types::ListPackFormulaAnalyticsResponse>> {
        self.limiter.analytics.until_ready().await;
        self.raw
            .list_pack_formula_analytics(pack_id, direction, limit, order_by, pack_formula_names, pack_formula_types, page_token, scale, since_date, until_date)
            .await
    }

    pub async fn get_analytics_last_updated<'a>(&'a self) -> Result<ResponseValue<types::AnalyticsLastUpdatedResponse>, Error<types::GetAnalyticsLastUpdatedResponse>> {
        self.limiter.analytics.until_ready().await;
        self.raw.get_analytics_last_updated().await
    }

    pub async fn list_workspace_members<'a>(&'a self, workspace_id: &'a str, included_roles: Option<&'a ::std::vec::Vec<types::WorkspaceUserRole>>, page_token: Option<&'a str>) -> Result<ResponseValue<types::WorkspaceMembersList>, Error<types::ListWorkspaceMembersResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_workspace_members(workspace_id, included_roles, page_token)
            .await
    }

    pub async fn change_user_role<'a>(&'a self, workspace_id: &'a str, body: &'a types::ChangeRole) -> Result<ResponseValue<types::ChangeRoleResult>, Error<types::ChangeUserRoleResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.change_user_role(workspace_id, body).await
    }

    pub async fn list_workspace_role_activity<'a>(&'a self, workspace_id: &'a str) -> Result<ResponseValue<types::GetWorkspaceRoleActivity>, Error<types::ListWorkspaceRoleActivityResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.list_workspace_role_activity(workspace_id).await
    }

    pub async fn list_packs<'a>(&'a self, access_type: Option<types::PackAccessType>, access_types: Option<&'a ::std::vec::Vec<types::PackAccessType>>, direction: Option<types::SortDirection>, exclude_individual_acls: Option<bool>, exclude_public_packs: Option<bool>, exclude_workspace_acls: Option<bool>, limit: Option<::std::num::NonZeroU64>, only_workspace_id: Option<&'a str>, pack_entrypoint: Option<types::PackEntrypoint>, page_token: Option<&'a str>, parent_workspace_ids: Option<&'a ::std::vec::Vec<::std::string::String>>, sort_by: Option<types::PacksSortBy>) -> Result<ResponseValue<types::PackSummaryList>, Error<types::ListPacksResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_packs(access_type, access_types, direction, exclude_individual_acls, exclude_public_packs, exclude_workspace_acls, limit, only_workspace_id, pack_entrypoint, page_token, parent_workspace_ids, sort_by)
            .await
    }

    pub async fn create_pack<'a>(&'a self, body: &'a types::CreatePackRequest) -> Result<ResponseValue<types::CreatePackResponse>, Error<types::CreatePackResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.create_pack(body).await
    }

    pub async fn get_pack<'a>(&'a self, pack_id: ::std::num::NonZeroU64) -> Result<ResponseValue<types::Pack>, Error<types::GetPackResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_pack(pack_id).await
    }

    pub async fn delete_pack<'a>(&'a self, pack_id: ::std::num::NonZeroU64) -> Result<ResponseValue<types::DeletePackResponse>, Error<types::DeletePackResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.delete_pack(pack_id).await
    }

    pub async fn update_pack<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::UpdatePackRequest) -> Result<ResponseValue<types::Pack>, Error<types::UpdatePackResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.update_pack(pack_id, body).await
    }

    pub async fn get_pack_configuration_schema<'a>(&'a self, pack_id: ::std::num::NonZeroU64) -> Result<ResponseValue<types::GetPackConfigurationJsonSchemaResponse>, Error<types::GetPackConfigurationSchemaResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_pack_configuration_schema(pack_id).await
    }

    pub async fn list_pack_versions<'a>(&'a self, pack_id: ::std::num::NonZeroU64, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>) -> Result<ResponseValue<types::PackVersionList>, Error<types::ListPackVersionsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_pack_versions(pack_id, limit, page_token)
            .await
    }

    pub async fn get_next_pack_version<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::GetNextPackVersionRequest) -> Result<ResponseValue<types::NextPackVersionInfo>, Error<types::GetNextPackVersionResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_next_pack_version(pack_id, body).await
    }

    pub async fn get_pack_version_diffs<'a>(&'a self, pack_id: ::std::num::NonZeroU64, base_pack_version: &'a str, target_pack_version: &'a str) -> Result<ResponseValue<types::PackVersionDiffs>, Error<types::GetPackVersionDiffsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .get_pack_version_diffs(pack_id, base_pack_version, target_pack_version)
            .await
    }

    pub async fn register_pack_version<'a>(&'a self, pack_id: ::std::num::NonZeroU64, pack_version: &'a str, body: &'a types::RegisterPackVersionRequest) -> Result<ResponseValue<types::PackVersionUploadInfo>, Error<types::RegisterPackVersionResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .register_pack_version(pack_id, pack_version, body)
            .await
    }

    pub async fn pack_version_upload_complete<'a>(&'a self, pack_id: ::std::num::NonZeroU64, pack_version: &'a str, body: &'a types::CreatePackVersionRequest) -> Result<ResponseValue<types::CreatePackVersionResponse>, Error<types::PackVersionUploadCompleteResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .pack_version_upload_complete(pack_id, pack_version, body)
            .await
    }

    pub async fn list_pack_releases<'a>(&'a self, pack_id: ::std::num::NonZeroU64, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>) -> Result<ResponseValue<types::PackReleaseList>, Error<types::ListPackReleasesResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_pack_releases(pack_id, limit, page_token)
            .await
    }

    pub async fn create_pack_release<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::CreatePackReleaseRequest) -> Result<ResponseValue<types::PackRelease>, Error<types::CreatePackReleaseResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.create_pack_release(pack_id, body).await
    }

    pub async fn update_pack_release<'a>(&'a self, pack_id: ::std::num::NonZeroU64, pack_release_id: ::std::num::NonZeroU64, body: &'a types::UpdatePackReleaseRequest) -> Result<ResponseValue<types::PackRelease>, Error<types::UpdatePackReleaseResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .update_pack_release(pack_id, pack_release_id, body)
            .await
    }

    pub async fn get_pack_oauth_config<'a>(&'a self, pack_id: ::std::num::NonZeroU64) -> Result<ResponseValue<types::PackOauthConfigMetadata>, Error<types::GetPackOauthConfigResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_pack_oauth_config(pack_id).await
    }

    pub async fn set_pack_oauth_config<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::SetPackOauthConfigRequest) -> Result<ResponseValue<types::PackOauthConfigMetadata>, Error<types::SetPackOauthConfigResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.set_pack_oauth_config(pack_id, body).await
    }

    pub async fn get_pack_system_connection<'a>(&'a self, pack_id: ::std::num::NonZeroU64) -> Result<ResponseValue<types::PackSystemConnectionMetadata>, Error<types::GetPackSystemConnectionResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_pack_system_connection(pack_id).await
    }

    pub async fn set_pack_system_connection<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::SetPackSystemConnectionRequest) -> Result<ResponseValue<types::PackSystemConnectionMetadata>, Error<types::SetPackSystemConnectionResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.set_pack_system_connection(pack_id, body).await
    }

    pub async fn patch_pack_system_connection<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::PatchPackSystemConnectionRequest) -> Result<ResponseValue<types::PackSystemConnectionMetadata>, Error<types::PatchPackSystemConnectionResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.patch_pack_system_connection(pack_id, body).await
    }

    pub async fn get_pack_permissions<'a>(&'a self, pack_id: ::std::num::NonZeroU64) -> Result<ResponseValue<types::PackPermissionList>, Error<types::GetPackPermissionsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_pack_permissions(pack_id).await
    }

    pub async fn add_pack_permission<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::AddPackPermissionRequest) -> Result<ResponseValue<types::AddPackPermissionResponse>, Error<types::AddPackPermissionResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.add_pack_permission(pack_id, body).await
    }

    pub async fn delete_pack_permission<'a>(&'a self, pack_id: ::std::num::NonZeroU64, permission_id: &'a str) -> Result<ResponseValue<types::DeletePackPermissionResponse>, Error<types::DeletePackPermissionResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .delete_pack_permission(pack_id, permission_id)
            .await
    }

    pub async fn list_user_pack_invitations<'a>(&'a self, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>) -> Result<ResponseValue<types::PackInvitationList>, Error<types::ListUserPackInvitationsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.list_user_pack_invitations(limit, page_token).await
    }

    pub async fn list_pack_invitations<'a>(&'a self, pack_id: ::std::num::NonZeroU64, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>) -> Result<ResponseValue<types::PackInvitationList>, Error<types::ListPackInvitationsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_pack_invitations(pack_id, limit, page_token)
            .await
    }

    pub async fn create_pack_invitation<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::CreatePackInvitationRequest) -> Result<ResponseValue<types::CreatePackInvitationResponse>, Error<types::CreatePackInvitationResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.create_pack_invitation(pack_id, body).await
    }

    pub async fn update_pack_invitation<'a>(&'a self, pack_id: ::std::num::NonZeroU64, invitation_id: &'a ::uuid::Uuid, body: &'a types::UpdatePackInvitationRequest) -> Result<ResponseValue<types::UpdatePackInvitationResponse>, Error<types::UpdatePackInvitationResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .update_pack_invitation(pack_id, invitation_id, body)
            .await
    }

    pub async fn delete_pack_invitation<'a>(&'a self, pack_id: ::std::num::NonZeroU64, invitation_id: &'a ::uuid::Uuid) -> Result<ResponseValue<types::DeletePackInvitationResponse>, Error<types::DeletePackInvitationResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .delete_pack_invitation(pack_id, invitation_id)
            .await
    }

    pub async fn reply_to_pack_invitation<'a>(&'a self, invitation_id: &'a ::uuid::Uuid, body: &'a types::HandlePackInvitationRequest) -> Result<ResponseValue<types::HandlePackInvitationResponse>, Error<types::ReplyToPackInvitationResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.reply_to_pack_invitation(invitation_id, body).await
    }

    pub async fn list_pack_makers<'a>(&'a self, pack_id: ::std::num::NonZeroU64) -> Result<ResponseValue<types::ListPackMakersResponse>, Error<types::ListPackMakersResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.list_pack_makers(pack_id).await
    }

    pub async fn add_pack_maker<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::AddPackMakerRequest) -> Result<ResponseValue<types::AddPackMakerResponse>, Error<types::AddPackMakerResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.add_pack_maker(pack_id, body).await
    }

    pub async fn delete_pack_maker<'a>(&'a self, pack_id: ::std::num::NonZeroU64, login_id: &'a str) -> Result<ResponseValue<types::DeletePackMakerResponse>, Error<types::DeletePackMakerResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.delete_pack_maker(pack_id, login_id).await
    }

    pub async fn list_pack_categories<'a>(&'a self, pack_id: ::std::num::NonZeroU64) -> Result<ResponseValue<types::ListPackCategoriesResponse>, Error<types::ListPackCategoriesResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.list_pack_categories(pack_id).await
    }

    pub async fn add_pack_category<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::AddPackCategoryRequest) -> Result<ResponseValue<types::AddPackCategoryResponse>, Error<types::AddPackCategoryResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.add_pack_category(pack_id, body).await
    }

    pub async fn delete_pack_category<'a>(&'a self, pack_id: ::std::num::NonZeroU64, category_name: &'a str) -> Result<ResponseValue<types::DeletePackCategoryResponse>, Error<types::DeletePackCategoryResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.delete_pack_category(pack_id, category_name).await
    }

    pub async fn upload_pack_asset<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::UploadPackAssetRequest) -> Result<ResponseValue<types::PackAssetUploadInfo>, Error<types::UploadPackAssetResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.upload_pack_asset(pack_id, body).await
    }

    pub async fn upload_pack_source_code<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::UploadPackSourceCodeRequest) -> Result<ResponseValue<types::PackSourceCodeUploadInfo>, Error<types::UploadPackSourceCodeResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.upload_pack_source_code(pack_id, body).await
    }

    pub async fn pack_asset_upload_complete<'a>(&'a self, pack_id: ::std::num::NonZeroU64, pack_asset_id: &'a str, pack_asset_type: types::PackAssetType) -> Result<ResponseValue<types::PackAssetUploadCompleteResponse>, Error<types::PackAssetUploadCompleteResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .pack_asset_upload_complete(pack_id, pack_asset_id, pack_asset_type)
            .await
    }

    pub async fn pack_source_code_upload_complete<'a>(&'a self, pack_id: ::std::num::NonZeroU64, pack_version: &'a str, body: &'a types::PackSourceCodeUploadCompleteRequest) -> Result<ResponseValue<types::PackSourceCodeUploadCompleteResponse>, Error<types::PackSourceCodeUploadCompleteResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .pack_source_code_upload_complete(pack_id, pack_version, body)
            .await
    }

    pub async fn get_pack_source_code<'a>(&'a self, pack_id: ::std::num::NonZeroU64, pack_version: &'a str) -> Result<ResponseValue<types::PackSourceCodeInfo>, Error<types::GetPackSourceCodeResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.get_pack_source_code(pack_id, pack_version).await
    }

    pub async fn list_pack_listings<'a>(&'a self, certified_agents_only: Option<bool>, direction: Option<types::SortDirection>, exclude_individual_acls: Option<bool>, exclude_public_packs: Option<bool>, exclude_workspace_acls: Option<bool>, install_context: Option<types::PackListingInstallContextType>, limit: Option<::std::num::NonZeroU64>, only_workspace_id: Option<&'a str>, order_by: Option<types::PackListingsSortBy>, pack_access_types: Option<&'a types::PackAccessTypes>, pack_entrypoint: Option<types::PackEntrypoint>, pack_ids: Option<&'a ::std::vec::Vec<i64>>, page_token: Option<&'a str>, parent_workspace_ids: Option<&'a ::std::vec::Vec<::std::string::String>>, sort_by: Option<types::PackListingsSortBy>) -> Result<ResponseValue<types::PackListingList>, Error<types::ListPackListingsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_pack_listings(certified_agents_only, direction, exclude_individual_acls, exclude_public_packs, exclude_workspace_acls, install_context, limit, only_workspace_id, order_by, pack_access_types, pack_entrypoint, pack_ids, page_token, parent_workspace_ids, sort_by)
            .await
    }

    pub async fn get_pack_listing<'a>(&'a self, pack_id: ::std::num::NonZeroU64, doc_id: Option<&'a str>, ingestion_id: Option<&'a str>, install_context: Option<types::PackListingInstallContextType>, release_channel: Option<types::IngestionPackReleaseChannel>, workspace_id: Option<&'a str>) -> Result<ResponseValue<types::PackListingDetail>, Error<types::GetPackListingResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .get_pack_listing(pack_id, doc_id, ingestion_id, install_context, release_channel, workspace_id)
            .await
    }

    pub async fn list_pack_logs<'a>(&'a self, pack_id: ::std::num::NonZeroU64, doc_id: &'a str, after_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, before_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, limit: Option<::std::num::NonZeroU64>, log_types: Option<&'a ::std::vec::Vec<types::PackLogType>>, order: Option<types::ListPackLogsOrder>, page_token: Option<&'a str>, q: Option<&'a str>, request_ids: Option<&'a ::std::vec::Vec<::std::string::String>>) -> Result<ResponseValue<types::PackLogsList>, Error<types::ListPackLogsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_pack_logs(pack_id, doc_id, after_timestamp, before_timestamp, limit, log_types, order, page_token, q, request_ids)
            .await
    }

    pub async fn list_ingestion_logs<'a>(&'a self, pack_id: ::std::num::NonZeroU64, tenant_id: &'a str, root_ingestion_id: &'a ::uuid::Uuid, after_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, before_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, ingestion_execution_id: Option<&'a ::uuid::Uuid>, ingestion_status: Option<types::IngestionStatus>, limit: Option<::std::num::NonZeroU64>, log_types: Option<&'a ::std::vec::Vec<types::PackLogType>>, only_execution_completions: Option<bool>, order: Option<types::ListIngestionLogsOrder>, page_token: Option<&'a str>, q: Option<&'a str>, request_ids: Option<&'a ::std::vec::Vec<::std::string::String>>) -> Result<ResponseValue<types::PackLogsList>, Error<types::ListIngestionLogsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_ingestion_logs(pack_id, tenant_id, root_ingestion_id, after_timestamp, before_timestamp, ingestion_execution_id, ingestion_status, limit, log_types, only_execution_completions, order, page_token, q, request_ids)
            .await
    }

    pub async fn list_grouped_pack_logs<'a>(&'a self, pack_id: ::std::num::NonZeroU64, doc_id: &'a str, after_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, before_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, limit: Option<::std::num::NonZeroU64>, order: Option<types::ListGroupedPackLogsOrder>, page_token: Option<&'a str>, q: Option<&'a str>) -> Result<ResponseValue<types::GroupedPackLogsList>, Error<types::ListGroupedPackLogsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_grouped_pack_logs(pack_id, doc_id, after_timestamp, before_timestamp, limit, order, page_token, q)
            .await
    }

    pub async fn list_grouped_ingestion_logs<'a>(&'a self, pack_id: ::std::num::NonZeroU64, tenant_id: &'a str, root_ingestion_id: &'a ::uuid::Uuid, after_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, before_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, ingestion_execution_id: Option<&'a ::uuid::Uuid>, limit: Option<::std::num::NonZeroU64>, order: Option<types::ListGroupedIngestionLogsOrder>, page_token: Option<&'a str>, q: Option<&'a str>) -> Result<ResponseValue<types::GroupedPackLogsList>, Error<types::ListGroupedIngestionLogsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_grouped_ingestion_logs(pack_id, tenant_id, root_ingestion_id, after_timestamp, before_timestamp, ingestion_execution_id, limit, order, page_token, q)
            .await
    }

    pub async fn list_ingestion_batch_executions<'a>(&'a self, pack_id: ::std::num::NonZeroU64, tenant_id: &'a str, root_ingestion_id: &'a ::uuid::Uuid, datasource: Option<&'a str>, execution_type: Option<types::IngestionExecutionType>, include_deleted_ingestions: Option<bool>, ingestion_execution_id: Option<&'a str>, ingestion_id: Option<&'a str>, ingestion_status: Option<types::IngestionStatus>, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>) -> Result<ResponseValue<types::IngestionBatchExecutionsList>, Error<types::ListIngestionBatchExecutionsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_ingestion_batch_executions(pack_id, tenant_id, root_ingestion_id, datasource, execution_type, include_deleted_ingestions, ingestion_execution_id, ingestion_id, ingestion_status, limit, page_token)
            .await
    }

    pub async fn list_ingestion_parent_items<'a>(&'a self, pack_id: ::std::num::NonZeroU64, tenant_id: &'a str, root_ingestion_id: &'a ::uuid::Uuid, ingestion_execution_id: &'a ::uuid::Uuid, ingestion_id: &'a ::uuid::Uuid, ingestion_status: Option<types::IngestionStatus>, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>) -> Result<ResponseValue<types::IngestionParentItemsList>, Error<types::ListIngestionParentItemsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_ingestion_parent_items(pack_id, tenant_id, root_ingestion_id, ingestion_execution_id, ingestion_id, ingestion_status, limit, page_token)
            .await
    }

    pub async fn get_pack_log_details<'a>(&'a self, pack_id: ::std::num::NonZeroU64, tenant_id: &'a str, root_ingestion_id: &'a ::uuid::Uuid, log_id: &'a str, details_key: &'a str) -> Result<ResponseValue<types::PackLogDetails>, Error<types::GetPackLogDetailsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .get_pack_log_details(pack_id, tenant_id, root_ingestion_id, log_id, details_key)
            .await
    }

    pub async fn list_pack_featured_docs<'a>(&'a self, pack_id: ::std::num::NonZeroU64) -> Result<ResponseValue<types::PackFeaturedDocsResponse>, Error<types::ListPackFeaturedDocsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw.list_pack_featured_docs(pack_id).await
    }

    pub async fn update_pack_featured_docs<'a>(&'a self, pack_id: ::std::num::NonZeroU64, body: &'a types::UpdatePackFeaturedDocsRequest) -> Result<ResponseValue<types::UpdatePackFeaturedDocsResponse>, Error<types::UpdatePackFeaturedDocsResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.update_pack_featured_docs(pack_id, body).await
    }

    pub async fn add_go_link<'a>(&'a self, organization_id: &'a str, body: &'a types::AddGoLinkRequest) -> Result<ResponseValue<types::AddGoLinkResult>, Error<types::AddGoLinkResponse>> {
        self.limiter.write.until_ready().await;
        self.raw.add_go_link(organization_id, body).await
    }

    pub async fn list_agent_session_ids<'a>(&'a self, tenant_id: &'a str, agent_instance_id: &'a ::uuid::Uuid, after_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, agent_session_id: Option<&'a ::uuid::Uuid>, before_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, limit: Option<::std::num::NonZeroU64>, log_types: Option<&'a ::std::vec::Vec<types::PackLogType>>, order: Option<types::ListAgentSessionIdsOrder>, page_token: Option<&'a str>, q: Option<&'a str>, request_ids: Option<&'a ::std::vec::Vec<::std::string::String>>) -> Result<ResponseValue<types::PackLogsList>, Error<types::ListAgentSessionIdsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_agent_session_ids(tenant_id, agent_instance_id, after_timestamp, agent_session_id, before_timestamp, limit, log_types, order, page_token, q, request_ids)
            .await
    }

    pub async fn list_agent_logs<'a>(&'a self, tenant_id: &'a str, agent_instance_id: &'a ::uuid::Uuid, after_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, agent_session_id: Option<&'a ::uuid::Uuid>, before_timestamp: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>, limit: Option<::std::num::NonZeroU64>, log_types: Option<&'a ::std::vec::Vec<types::PackLogType>>, order: Option<types::ListAgentLogsOrder>, page_token: Option<&'a str>, q: Option<&'a str>, request_ids: Option<&'a ::std::vec::Vec<::std::string::String>>) -> Result<ResponseValue<types::PackLogsList>, Error<types::ListAgentLogsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_agent_logs(tenant_id, agent_instance_id, after_timestamp, agent_session_id, before_timestamp, limit, log_types, order, page_token, q, request_ids)
            .await
    }

    pub async fn get_agent_pack_log_details<'a>(&'a self, tenant_id: &'a str, agent_instance_id: &'a ::uuid::Uuid, log_id: &'a str, details_key: &'a str) -> Result<ResponseValue<types::PackLogDetails>, Error<types::GetAgentPackLogDetailsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .get_agent_pack_log_details(tenant_id, agent_instance_id, log_id, details_key)
            .await
    }
    pub async fn table_refs(&self, doc_id: &str) -> Result<Vec<TableReference>, Error<types::ListTablesResponse>> {
        paginate_all(move |page_token| async move {
            self.list_tables(doc_id, None, page_token.as_deref(), None, None)
                .await
                .map(|response| response.into_inner())
        })
        .await
    }

    pub async fn tables(&self, doc_id: &str) -> Result<Vec<Table>, ClientTablesError> {
        use ClientTablesError::*;
        let table_refs = handle!(self.table_refs(doc_id).await, ListTablesFailed);
        let mut all_tables = Vec::new();
        for table_ref in table_refs {
            let table_response = handle!(self.get_table(doc_id, &table_ref.id, None).await, GetTableFailed);
            all_tables.push(table_response.into_inner());
        }
        Ok(all_tables)
    }

    pub async fn columns_map(&self, doc_id: &str, table_ids: impl IntoIterator<Item = TableId>) -> Result<HashMap<TableId, Vec<Column>>, Error<types::ListColumnsResponse>> {
        let mut columns_map = HashMap::new();

        for table_id in table_ids {
            let table_id_ref = table_id.as_ref();
            let columns = paginate_all(move |page_token| async move {
                self.list_columns(doc_id, table_id_ref, None, page_token.as_deref(), None)
                    .await
                    .map(|response| response.into_inner())
            })
            .await?;

            columns_map.insert(table_id, columns);
        }

        Ok(columns_map)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn rows(&self, doc_id: &str, table_id: &str, query: Option<&str>, sort_by: Option<types::RowsSortBy>, sync_token: Option<&str>, use_column_names: Option<bool>, value_format: Option<types::ValueFormat>) -> Result<Vec<Row>, Error<types::ListRowsResponse>> {
        paginate_all(move |page_token| async move {
            self.list_rows(doc_id, table_id, None, page_token.as_deref(), query, sort_by, sync_token, use_column_names, value_format, None)
                .await
                .map(|response| response.into_inner())
        })
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list_rows_rich<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, limit: Option<::std::num::NonZeroU64>, page_token: Option<&'a str>, query: Option<&'a str>, sort_by: Option<types::RowsSortBy>, sync_token: Option<&'a str>, use_column_names: Option<bool>, visible_only: Option<bool>) -> Result<ResponseValue<RichRowList>, Error<types::ListRowsResponse>> {
        self.limiter.read.until_ready().await;
        self.raw
            .list_rows_rich(doc_id, table_id_or_name, limit, page_token, query, sort_by, sync_token, use_column_names, visible_only)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn rows_rich(&self, doc_id: &str, table_id: &str, query: Option<&str>, sort_by: Option<types::RowsSortBy>, sync_token: Option<&str>, use_column_names: Option<bool>, visible_only: Option<bool>) -> Result<Vec<RichRow>, Error<types::ListRowsResponse>> {
        paginate_all(move |page_token| async move {
            self.list_rows_rich(doc_id, table_id, None, page_token.as_deref(), query, sort_by, sync_token, use_column_names, visible_only)
                .await
                .map(|response| response.into_inner())
        })
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn rows_map(&self, doc_id: &str, table_ids: impl IntoIterator<Item = TableId>, query: Option<&str>, sort_by: Option<types::RowsSortBy>, sync_token: Option<&str>, use_column_names: Option<bool>, value_format: Option<types::ValueFormat>) -> Result<HashMap<TableId, Vec<Row>>, Error<types::ListRowsResponse>> {
        let rows_futures = table_ids.into_iter().map(|table_id| async move {
            let rows = self
                .rows(doc_id, &table_id, query, sort_by, sync_token, use_column_names, value_format)
                .await?;
            Ok::<(TableId, Vec<Row>), Error<types::ListRowsResponse>>((table_id, rows))
        });

        let mut rows_map = HashMap::new();

        for future in rows_futures {
            let (table_id, rows) = future.await?;
            rows_map.insert(table_id, rows);
        }

        Ok(rows_map)
    }

    pub async fn upsert_rows_correct<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, disable_parsing: Option<bool>, body: &'a types::RowsUpsert) -> Result<ResponseValue<RowsUpsertResultCorrect>, Error<types::UpsertRowsResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .upsert_rows_correct(doc_id, table_id_or_name, disable_parsing, body)
            .await
    }

    pub async fn update_row_correct<'a>(&'a self, doc_id: &'a str, table_id_or_name: &'a str, row_id_or_name: &'a str, disable_parsing: Option<bool>, body: &'a types::RowUpdate) -> Result<ResponseValue<RowUpdateResultCorrect>, Error<types::UpdateRowResponse>> {
        self.limiter.write.until_ready().await;
        self.raw
            .update_row_correct(doc_id, table_id_or_name, row_id_or_name, disable_parsing, body)
            .await
    }
}
