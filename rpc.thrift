namespace php RPC

// global

enum IndustryID
{
  AUTOBODY = 1,
  GLASS = 2,
  ERS = 3,
  DEALER = 5,
}

enum Product
{
  ProductStatus = 1,
  ProductPostDelivery = 2,
  ProductCSIShop = 3,
  ProductCSIInsurance = 4,
  ProductDataShare = 5,
  ProductRebuttal = 6,
  ProductReviews = 7,
  ProductListen = 8,
  ProductCustomerCommunication = 9,
}

enum ManagementSystem
{
  CCC1 = 5,
  CCC1_Estimating = 8,
  CCC1_Repair_Workflow = 9,
  CDK = 10,
  DealerBuilt = 11,
  Manual_Mode = 14,
  DealerTrack = 12,
  Autosoft_Mix = 20,
  Dominion_Vue_20 = 22,
}

// test drives

struct TestDrive
{
  1:string company_name,
  2:string first_name,
  3:string last_name,
  4:string mobile_phone,
  5:string email,
  6:string remote_ip,
}

service TestDriveService
{
  void save(1:TestDrive test_drive)
}

// promo code

struct PromoCode
{
  1: string code
  2: string description
  3: double price
}

exception PromoCodeNotFound
{
}

service PromoCodeService
{
  PromoCode getByCode(1:string code) throws (1:PromoCodeNotFound notfound)
}

// registration

struct CompanyHours
{
  1:string open,
  2:string close,
}

struct CompanySchedule
{
  1:CompanyHours sun,
  2:CompanyHours mon,
  3:CompanyHours tue,
  4:CompanyHours wed,
  5:CompanyHours thu,
  6:CompanyHours fri,
  7:CompanyHours sat,
}

struct Registration
{
  1:IndustryID industry_id,

  2:string company_name,
  3:string company_phone,
  4:string company_address,
  5:string company_city,
  6:string company_region,
  7:string company_postal_code,
  8:string company_country,
  9:CompanySchedule company_schedule,
  10:string time_zone,

  11:string first_name,
  12:string last_name,
  13:string email,
  14:string user_name,
  15:string password,

  16:string promo_code,
  17:list<Product> products,
  18:string subscription_id,
  19:string auth_code,
  20:string transaction_id,
  21:string session_token,
}

service RegistrationService
{
  void save(1:Registration registration)
}

// company

enum PaperSize
{
  Letter = 1,
  Legal = 2,
}

struct OEMInformation
{
  1:i32 iAPIUserID,
  2:string sCompanyName,
  3:i32 iOEMAPIID,
  4:string sExternalID,
  5:bool bEnabled,
  6:string sMake,
  7:string sBranding,
}

struct VehicleHistoryDisplayOption
{
  1:bool is_shown,
  2:bool is_dms_shown,
  3:bool is_oem_shown
}

struct ManufactureCampaignDisplayOption
{
  1:bool is_shown,
  2:bool is_dms_shown,
  3:bool is_oem_shown,
  4:bool is_motor_shown
}

struct IndividualServiceDisplayOption
{
  1:bool is_shown
  2:bool is_express_shown
  3:bool is_repairs_shown
  4:bool is_warranty_shown
}

struct ServiceRequestDisplayOption
{
  1:bool is_shown
}

struct RecommendedServiceDisplayOption
{
  1:bool is_shown
}

struct VehicleInspectionDisplayOption
{
  1:bool is_shown,
  2:bool is_tire_warranty_shown
}

struct DeferredServiceDisplayOption
{
  1:bool is_shown
}

struct DisplayOption
{
  1:VehicleHistoryDisplayOption vehicle_history,
  2:ManufactureCampaignDisplayOption manufacture_campaign,
  3:IndividualServiceDisplayOption individual_service,
  4:ServiceRequestDisplayOption service_request
  5:RecommendedServiceDisplayOption recommended_service,
  6:VehicleInspectionDisplayOption vehicle_inspection,
  7:DeferredServiceDisplayOption deferred_service
}

struct DisplayOptions
{
  1:DisplayOption appointment,
  2:DisplayOption vci,
  3:DisplayOption mpi
}

struct CompanyID
{
  1:string label,
  2:string external_id,
}

struct Company
{
  1:i32 id,
  2:string company_name,
  3:bool is_oem_available,
  4:PaperSize vci_paper_size,
  5:list<OEMInformation> oem_information,
  6:i32 appointment_reminder_minutes,
  7:list<CompanyID> external_ids,
  8:i32 new_to_widget
}

exception CompanyNotFound
{
}

struct SocialNetworkLink{
  1:i32 company_id,
  2:string url,
  3:string url_type,
  4:string shop_url
}

struct CompanyUpdate
{
  1:PaperSize vci_paper_size,
  2:i32 appointment_reminder_minutes,
}

struct NotificationMessage
{
  1:i32 iNotificationMessageID,
  2:i32 iProgressRegionID,
  3:i32 iPromiseTypeID,
  4:string sBody,
  5:string sHTMLBody,
  6:i32 iExternalID,
  7:i32 iDisplayOrder,
  8:i32 iCompanyID,
  9:i32 iDaysAfterEvent,
  10:string sSubject
}

struct CDKModelMap
{
  1:string motor_model,
  2:string cdk_make,
  3:string cdk_model
}

struct CDKMakeMap
{
  1:string motor_make,
  2:list<CDKModelMap> model_mapping
}

struct MfrCodeMap
{
  1:string make,
  2:string mfrcode,
}

struct DMSPartPriceOption
{
  1:i32 id,
  2:string label,
  3:string attribute
}

service CompanyService
{
  Company getByName(1:i32 industry_id, 2:string name)  throws (1:CompanyNotFound notfound),
  list<string> getCompanyServices(1:i32 company_id),
  list<SocialNetworkLink> getCompanySocialNetworkLinks(1:i32 company_id),
  string getCompanyInfo(1:i32 company_id),
  list<string> usersInCompany(1:i32 company_id),
  void queueCompanyHistoryPull(1:i32 company_id),
  Company getById(1:i32 company_id),
  void update(1:i32 company_id, 2:CompanyUpdate data),
  DisplayOptions getDisplayOptions(1:i32 company_id),
  void setDisplayOptions(1:i32 company_id, 2:DisplayOptions data),
  OEMInformation get_company_oem_setting(1:i32 company_id, 2:i32 oem_api_id),
  void enableCompanyOEMSetting(1:i32 company_id, 2:i32 oem_api_id, 3:string external_id),
  void disableCompanyOEMSetting(1:i32 company_id, 2:i32 oem_api_id),
  list<OEMInformation> getCompanyOEMSettings(1:i32 company_id),
  list<OEMInformation> getCompanyOEMs(1:i32 company_id),
  NotificationMessage getNotificationMessage(1:i32 company_id, 2:i32 promise_type, 3:i32 promise_region),
  void setNotificationMessage(1:NotificationMessage notification_message),
  list<CDKMakeMap> getCDKMakeModelMapping(1:i32 company_id),
  void setCDKMakeModelMapping(1:i32 company_id, 2:list<CDKMakeMap> make_model_mapping),
  list<MfrCodeMap> getMfrCodeMapping(1:i32 company_id),
  void setMfrCodeMapping(1:i32 company_id, 2:list<MfrCodeMap> mfrcode_mapping),
  list<PriceCode> getPriceCodes(1:i32 company_id),
  list<string> getDispatchCodes(1:i32 company_id),
  list<DMSPartPriceOption> getPartPriceOptions(1:i32 company_id)
}

// user

enum UserRole
{
  Admin = 1,
  Level1 = 2,
  Normal = 3,
}

struct User
{
  1: i32 id,
  2: string full_name,
  4: UserRole role,
  5: bool live_status,
  6: bool active_status,
  7: string email,
  8: string mobile_phone,
  9: i32 company_id,
  10:string away_message,
  11:string nickname,
  12:string role_preset,
  13:string role_label,
  14:list<string> permissions,
}

struct NewAppointment
{
  1:string sTimestamp,
  2:optional i64 iPromiseID,
  3:i32 iCompanyID,
  4:i64 iLoginID, // maps to created by user id and transitions
  5:i64 iCustomerID,
  6:i64 iVehicleID,
  7:string sService,
  8:string sConcerns,
  9:string sNotes,
  10:string sQRCode,
  11:string sStatus,
  12:optional string sExternalID,
  13:optional i32 iSourceID,
  14:string sTransportOption,
  15:optional i32 iAPIUserID,
  16:optional string sCommPreference,
  17:optional string sUniqueIdentifier,
  18:i32 iMileage,
  19:i32 fFuelLevel,
  20:i32 iTechnicianID,
  21:string dtPromised,
  23:i32 iOriginalCreatedByLoginID, // maps to created by login id does not change
}

struct ScheduleOptions
{
  1:string sRangeType,
  2:string sStart,
  3:string sEnd,
  4:string sType,
}

struct UserSchedule
{
  1:i32 iScheduleID,
  2:i32 iRecurrenceID,
  3:i32 iLoginID,
  4:string sType,
  5:string sStartTime,
  6:string sEndTime,
  7:string sStartDate,
  8:string sEndDate,
  9:string sRecurringType,
  10:i32 iRecurringInterval,
  11:i32 iRecurringDayOfTheMonth,
  12:bool bRecurringSun,
  13:bool bRecurringMon,
  14:bool bRecurringTue,
  15:bool bRecurringWed,
  16:bool bRecurringThu,
  17:bool bRecurringFri,
  18:bool bRecurringSat,
  19:i32 iPromiseID,
  20:string sScheduleName,
  21:string sColor,
  22:i32 iScheduleGroupID,
  23:i32 appt_changed_by_login_id,
  24:bool appt_changed_to_confirmed,
  25:bool appt_datetime_changed,
  26:bool bIsOrdinal,
  27:string sOrdinalRecurringSequence,
  28:string sOrdinalRecurringDay,
  29:bool bEnabled,
  30:bool bIsHoliday,
  31:bool bIsManualHoliday,
  32:bool bNoEndDate,
}

struct UserAssociatedCompany
{
  1:i32 iCompanyID,
  2:string sCompanyName,
}

struct UserCompany
{
  1:i32 iCompanyID,
  2:i32 iCompanyCount,
  3:string sCompanyName,
  4:i32 iInsuranceCount,
  5:i32 iUserCount,
  6:i32 iLockedCount,
  7:string sCompanyAddress,
  8:string sCompanyCity,
  9:string sCompanyState,
  10:string sCompanyPostalCode,
  11:i32 iCompanyTypeID,
  12:string sManagementSystem,
  13:bool iEditable,
}

struct ScheduleResponse
{
  1:UserSchedule schedule,
  2:NewAppointment appointment,
}

struct SetScheduleResponse
{
  1:string status_code,
  2:string status_message,
  3:list<UserSchedule> overlapping_records,
  4:i32 schedule_id,
  5:bool status,
  6:i32 appointment_id,
  7:i32 recurrence_id,
}

struct UserCompanyLogin
{
  1:i32 iLoginID,
  2:string sUserID,
  3:string sEmailAddress,
  4:string sCompanyName,
  5:string sFullName,
  6:i32 iLoginTypeID,
  7:string sParentUserID,
  8:i32 iDefaultFlag,
  9:i32 iReceptionistFlag,
  10:i32 iActiveFlag
  11:i32 iLiveFlag,
  12:i32 iLoginEnabled,
  13:string dtLockedDateTime,
  14:i32 iChildCount,
  15:string dtLastActivityTimeStamp,
  16:string dtLastWelcomeEmailSent,
  17:string dtLastTrainingEmailSent,
  18:i32 iInsuranceCount,
  19:i32 iCompanyCount,
  20:string sUserMetaData,
}

exception UserNotFound
{
}

struct UserAlertSetting
{
  1:i32 alert_id,
  2:string description,
  3:bool text_enabled,
  4:bool email_enabled,
  5:bool web_enabled,
  6:bool app_enabled,
}

service UserService
{
  User getById(1:i32 user_id) throws (1:UserNotFound notfound),
  User getByUserName(1:string user_name) throws (1:UserNotFound notfound),
  bool saveUser(1:User user),
  void resendWelcomeEmail(1:i32 login_id, 2:bool force),
  i32 getUserCompaniesLoginCount(1:i32 industry_id, 2:i32 login_id, 3:string search, 4:string states),
  i32 getUserCompaniesCount(1:i32 industry_id, 2:i32 login_id, 3:string search, 4:string states),
  list<UserCompany> getUserCompanies(1:i32 industry_id, 2:i32 login_id, 3:string search, 4:string states, 5:i32 rows_start,
                              6:i32 rows_per_page),
  list<UserCompanyLogin> getUserCompanyLogins(1:i32 company_id, 2:i32 login_id),
  map<string,i32> getAccuracyRating(1:i32 login_id),
  bool companyUserAccess(1:i32 company_id, 2:i32 login_id),
  bool companyDataAccess(1:i32 company_id, 2:i32 login_id),
  bool receptionistCheck(1:i32 user_id),
  list<string> associatedUsers(1:i32 user_id),
  list<UserAssociatedCompany> associatedCompanies(1:i32 user_id),
  list<UserAlertSetting> getAlertSettings(1:i32 login_id),
  bool saveAlertSettings(1:i32 login_id, 2:list<UserAlertSetting> alert_settings),
  list<User> getUsersByCompanyID(1:i32 company_id),
  list<User> getUsersByLoginID(1:i32 login_id),
  list<User> getUsersByType(1:i32 company_id, 2:string user_type),
}

struct TimeInterval
{
  1:list<i32> user_ids,
  2:string start_time,
  3:string end_time,
}

struct DateInterval
{
  1:map<string, TimeInterval> time_intervals,
  2:string start_date,
  3:string end_date,
}

struct Availability
{
  1:map<string, DateInterval> date_intervals
}

service UserScheduleService
{
  list<UserSchedule> getUserSchedule(1:i32 user_id, 2:ScheduleOptions options),
  SetScheduleResponse setUserSchedule(1:UserSchedule s, 2:NewAppointment appointment, 3:string action, 4:bool recurrence, 5:bool alter),
  void deleteUserSchedule(1:i32 schedule_id, 2:bool recurrence),
  i32 getUnscheduledID(1:i32 company_id),
  Availability firstAvailable(1:UserSchedule schedule, 2:i32 increment, 3:i32 limit),
  list<string> getDates(1:UserSchedule schedule, 2:string start_date, 3:string end_date, 4:string generate_type),
}

// Queue
service QueueService
{
  void queueCustomer(1:i32 customer_id),
  void queueCompanyHistoryPull(1:i32 company_id),
  void queueCustomerVehicle(1:i32 customer_id, 2:i32 vehicle_id),
}

// session

struct Session
{
  1: string token,
  2: i32 expires_in,
  3: i32 user_id,
  4: i32 company_id,
  5: string jwt,
}

enum LoginErrorCode
{
  UserNotFound = 1,
  UserInactive = 2,
  UserLocked = 3,
  BadPassword = 4,
  ExpiredPassword = 5,
  LoginDisabled = 6,
  MFARequired = 7,
  MFAInvalidToken = 8,
}


struct LoginResponse
{
  1:i32 user_id,
  2:string token,
  3:string jwt,
  4:LoginErrorCode error_code,
  5:i32 company_id,
}

exception SessionNotFound
{
}

exception PasswordValidationException
{
  1:string message,
}

service SessionService
{
  Session getByToken(1:string token) throws (1:SessionNotFound notfound),
  LoginResponse login(1:string username, 2:string password, 3:MFAMethod mfa_contact_method, 4:string mfa_token, 5:string device_fingerprint,),
  void logout(1:string token),
  void setDeviceToken(1:string session_token, 2:string device_token),
  LoginResponse getByAccess(1:string location_id, 2:string user_external_id, 3:string access_token),
  bool setPassword(1:i32 login_id, 2:string old_password, 3:string new_password) throws (1:PasswordValidationException exc),
 string create(1:i32 login_id, 2:string device_id, 3:string device_token),
}

// vci service

struct TireInformation
{
  1:string brand,
  2:string size,
  3:i32 left_front_indicator,
  4:i32 left_front_depth,
  5:i32 right_front_indicator,
  6:i32 right_front_depth,
  7:i32 left_rear_indicator,
  8:i32 left_rear_depth,
  9:i32 right_rear_indicator,
  10:i32 right_rear_depth,
}

service VciService
{
  TireInformation getTireInformation(1:i32 promise_id),
  void setTireInformation(1:i32 promise_id, 2:TireInformation tire_info),
}

// promise

struct PromiseXML
{
    // Company
    1:i32 iLocationID,
    2:string sName,
    3:string sTimeZone,

    // Employee
    4:i32 iEmployeeID,
    5:string sEmployeeFirstName,
    6:string sEmployeeLastName,

    // Customer
    7:i32 iCustomerID,
    8:string sCustomerFirstName,
    9:string sCustomerLastName,
    10:string sCustomerEmail,
    11:string sCustomerMobilePhone,
    12:string sCustomerHomePhone,
    13:string sCustomerWorkPhone,
    14:string sCustomerCommPreference,

    15:i32 iPromiseID,
    16:string dtPromiseDateAppointment,

    // Vehicle
    17:i32 iVehicleID,
    18:string VehicleMake,
    19:string VehicleModel,
    20:string VehicleYear,
    21:string VehicleVIN,
    22:string VehicleLicense,
    23:string VehicleTrim,
    24:string VehicleOdometer,

    30:string sPromiseAppointmentStatus,
    31:string sPromiseTransportationOptions,
    32:string sPromiseAppointmentNotes,

    33:i32 iDeleteFlag,

    34:i32 iServiceForm,
}

struct Promise
{
  1:i32 iPromiseID
  9:i32 iPromiseTypeID,

  11:i32 iAPIUserID,
  12:i32 iInsuranceCompanyID,
  13:string sClaimNumber,
  14:string sMessageTriggers,
  15:i32 iCancelFlag,
  16:string sUniqueIdentifier,
  17:string sExternalURL,
  18:i32 iDataShare,
  19:i32 iSendPDF,
  20:i32 iSendCSI,
  21:i32 iSendSR,
  22:i32 iSendPDN,
  23:i32 iMobilePhoneInitialFlag,
  24:i32 iEmailInitialFlag,
  25:i32 iStopFlag,
  26:string dtFollowUpCall,
  27:i32 iFollowUpReminder,
  28:string dtAppointment,
  29:i32 iAlternateCustomerID,
  30:string sConcerns,

  100:string autobody_sClaimLossType,
  101:bool autobody_iAssignmentFlag,
  102:string autobody_sService,
  103:string autobody_dtAssignment,
  104:string autobody_dtCreated,
  105:string autobody_dtPromised,
  106:string autobody_dtCompleted,
  107:string autobody_dtDelivered,
  108:string autobody_sVehicleMake,
  109:i32 autobody_iCurrProgressPct,
  110:string autobody_sInsuranceAgent,
  111:string autobody_dtClaimLoss,
  112:string autobody_sRentalCompany,
  113:string autobody_sOriginsuranceCompany,
  114:double autobody_fPrice,
  115:string autobody_sTechBodyName,
  116:string autobody_sTechPaintName,
  117:string autobody_sServiceRepName,
  118:string autobody_sRepairOrderNumber,
  119:string autobody_sUniqueFileIdentifier,
  120:string autobody_dtOrigPromised,

  200:string glass_dtQuoted,
  201:string glass_dtAppointment,
  202:i32 glass_iPartsOK,
  203:bool glass_iCompletable,
  204:string glass_dtCompleted,
  205:string glass_sVehicleMake,

  300:string ers_dtAppointment,
  301:bool ers_iCompletable,
  302:string ers_dtCompleted,
  303:string ers_sVehicleMake,

  400:string dealer_dtCreated,
  401:string dealer_dtPromised,
  402:string dealer_dtCompleted,
  403:string dealer_dtNotify,
  404:string dealer_dtDelivered,
  405:string dealer_sVehicleMake,
  406:i32 dealer_iCurrProgressPct,
  407:bool dealer_iExpressFlag,
  408:string dealer_sVehicleModel,
  409:i32 dealer_iVehicleYear,
  410:string dealer_dtOrigPromised,
  411:i32 dealer_iMileage,
  412:double dealer_fFuelLevel,
  413:string dealer_sVehicleVIN,
  414:string dealer_sVehicleLicense,
  415:string dealer_sVehicleTrim,
  416:i32 iTechnicianID,
  417:string sTechnician,
  418:i32 iProductionStatusTypeID,
  419:string sProductionStatusName,
  420:string sProductionStatusColor,

  500:i32 iVehicleID,

  600:i32 iCustomerID,
  601:string sCustomerFName,
  602:string sCustomerLName,
  603:string sMobilePhone,
  604:string sHomePhone,
  605:string sWorkPhone,
  606:string sEmail,
  607:string sCommPreference,

  700:string sNotes,
  701:string sStatus,
  702:string sTransportationOption,
  703:string sService,
  704:i32 iSourceID,

  800:i32 iCreatedByLoginID,
  801:i32 sEmployeeFirstName,
  802:i32 sEmployeeLastName,

  900:i32 iCompanyID,
  901:i32 iLocationID,
  902:string sLocationName,
  903:string sLocationTimeZone,

  1000:i32 iDeleteFlag,

  1100:string sHangTag
}

struct PromiseCompany
{
  1:i32 id,
  2:string name,
  3:i32 promise_count,
  4:i32 active_count,
  5:bool new_reply,
  6:string address,
  7:string city,
  8:string state,
  9:string postal_code,
}

struct PromiseCompanies
{
  1:bool has_more,
  2:list<PromiseCompany> companies,
}

struct PromiseUser
{
  1:i32 user_id,
  2:string full_name,
  3:i32 promise_count,
  4:i32 active_count,
  5:bool new_reply,
  6:i32 company_id,
  7:string company_name,
}

enum ListPromiseStatus
{
  MPI_REVIEW,
  PENDING,
  REVIEW,
  SENT,
  VIEWED,
  REPLIED,
}

struct ListPromise
{
  1:Promise promise,
  2:string filter,
  3:string api_name,
  4:bool new_reply,
  5:i32 notification_count,
  6:bool csi_completed,
  7:string time_zone,
  8:string company_schedule,
 10:ListPromiseStatus attachment_status,
 11:string technician
}

struct PromisePromises
{
  1:i32 promise_count,
  2:list<ListPromise> promises,
}

enum PromiseSearchFields
{
  CompanyName = 1,
  EmployeeName = 2,
  CustomerName = 3,
  CustomerEmail = 4,
  CustomerPhone = 5,
  ClaimNumber = 6,
  UniqueIdentifier = 7,
  VehicleIdenfication = 8,
}

exception PromiseNotFound
{
}

exception ROImportError
{
  1:string message,
}

struct Attachment
{
  1:i32 iPromiseAttachmentID,
  2:string sTempName,
  3:string sMIMEType,
  4:string sFileName,
  5:string sFileGUID,
}

struct PromiseHistory
{
  1:string type,
  2:string message,
  3:string file_guid,
  4:string source,
  5:string destination,
  6:string source_type,
  7:string source_comm,
  8:string destination_type,
  9:string destination_comm,
  10:string created_at,
  11:string meta_data,
}

service PromiseService
{
  Promise getById(1:i32 industry_id, 2:i32 promise_id) throws (1:PromiseNotFound notfound),
  list<PromiseHistory> getHistory(1:i32 promise_id),
  map<string, string> validate(1:i32 industry_id, 2:Promise promise),
  i32 save(1:i32 industry_id, 2:Promise promise, 3:string time_zone, 4:bool from_api, 5:i32 from_user_id, 6:string from_user_fullname),
  void deletePromise(1:i32 industry_id, 2:Promise promise, 3:bool queue),
  bool cancel(1:i32 promise_id),
  bool uncancel(1:i32 promise_id),
  void queue(1:i32 industry_id, 2:Promise promise),
  map<string, string> queueInternalPromise(1:Promise promise),
  map<string, string> getTokens(1:i32 promise_id),
  string replaceTokens(1:i32 promise_id, 2:string message, 3:map<string,string> extra_params),
  string getUdpPaymentDisclosure(1:i32 promise_id),
  PromiseCompanies getPromiseCompanies(1:i32 login_id, 2:i32 insurance_company_id,
                                       3:i32 campaign_id,
                                       4:set<string> filters, 5:set<string> states,
                                       6:string search, 7:i32 row_start, 8:i32 rows_per_page, 9:PromiseSearchFields search_field,
                                       10:list<i32> login_filters),
  list<PromiseUser> getPromiseUsers(1:i32 login_id, 2:list<i32> company_id,
                                    3:i32 insurance_company_id,
                                    4:i32 campaign_id,
                                    5:set<string> filters,
                                    6:string search, 7:PromiseSearchFields search_field, 8:list<i32> login_filters),
  list<PromiseUser> getPromiseUsersFromCompanies(1:i32 login_id,
                                                 2:list<i32> company_ids,
                                                 3:i32 insurance_company_id,
                                                 4:i32 campaign_id,
                                                 5:set<string> filters,
                                                 6:string search,
                                                 7:PromiseSearchFields search_field,
                                                 8:list<i32> login_filters),
  PromisePromises getPromises(1:i32 login_id,
                            2:list<i32> user_id,
                            3:list<i32> technician_id,
                            4:list<string> production_status,
                            5:i32 insurance_company_id,
                            6:i32 campaign_id,
                            7:set<string> filters,
                            8:string search,
                            9:PromiseSearchFields search_field,
                            10:string sort_field,
                            11:bool sort_descending,
                            12:i32 row_start,
                            13:i32 rows_per_page),
  PromisePromises getPromisesFromUsers(1:i32 login_id,
                                       2:list<i32> user_ids,
                                       3:i32 insurance_company_id,
                                       4:i32 campaign_id,
                                       5:set<string> filters,
                                       6:string search,
                                       7:PromiseSearchFields search_field,
                                       8:string sort_field,
                                       9:bool sort_descending,
                                       10:i32 row_start,
                                       11:i32 rows_per_page),
  i32 logEvent(1:i32 promise_id, 2:string description, 3:bool prevent_dups, 4:i32 dup_minutes_apart,
               5:i32 double_opt_in_reply_flag, 6:string meta_data),
  bool sendCustomerMessage(1:i32 promise_id, 2:Transcript transcript, 3:string subject, 4:string body,
                           5:string html_subject, 6:string html_body, 7:list<Attachment> attachments, 8:bool force),
  bool updateCustomer(1:i32 promise_id, 2:i32 progress_region_id, 3:string extra_log_entry, 4:map<string, string> extra_params),
  bool promiseUserAccess(1:i32 promise_id, 2:i32 login_id),
  void stop(1:i32 promise_id, 2:i32 method, 3:i32 trancsript_id, 4:string reason, 5:bool send_confirmation),
  void addSocialLinkClick(1:i32 promise_id, 2:string url_type),
  bool socialLinkClicked(1:i32 promise_id, 2:string url_type),
  i32 importRepairOrder(1:i32 login_id, 2:string external_job_id) throws(1:ROImportError error),
  bool getUnreadStatus(1:i32 login_id),
  bool markAsRead(1:i32 promise_id, 2:i32 login_id),
  bool lockAPI(1:i32 promise_id, 2:i32 login_id, 3:optional string message),
  bool isAPILocked(1:i32 promise_id, 2:i32 login_id, 3:optional string message),
  bool unlockAPI(1:i32 promise_id, 2:i32 login_id, 3:optional string subject),
  i32 getByUniqueIdentifier(1:string unique_id, 2:i32 login_id),
  i32 getByClaimNumber(1:string claim_number, 2:i32 login_id),
}

enum UserFlag
{
  FlagNormal = 1,
  FlagManager = 2,
  FlagCorporate = 4,
  FlagInsurance = 8,
  FlagReceptionist = 16,
  FlagTechnician = 32,
  FlagPartsUser = 64,
  FlagCarWashUser = 128,
  FlagPartsManager = 256,
  FlagCallCenterRep = 512,
  FlagNormalManager = 1024,
  FlagShopForman = 2048,
  FlagServiceAdvisor = 4096,
}

enum AlertType
{
  TypeSQSurvey = 1,
  TypePromise48HR = 2,
  TypeCSIShop = 3,
  TypePromiseNoPhone = 4,
  TypePromiseInvalidEmail = 5,
  TypePromiseMissingEmail = 6,
  TypeCustomerReply = 9,
  TypeLowCommentSentiment = 10,
  TypeDuplicateContactInfo = 11,
  TypeServiceRecoveryComplete = 16,
  TypePendingAttachment = 17,
  TypeMPIFormComplete = 20,
  ProductionStatus = 21,
  TypeTechnicianAssigned = 24,
  TypeAppointmentCreated = 25,
  TypeAppointmentCreatedWidget = 26,
  TypeTransportationAppointment = 27,
  TypeOpCodeAppointment = 28,
  TypeRecallAppointment = 29,
  TypeUserTagging = 30,
}

enum LoginCompanyType
{
  CompanyVisibility = 1,
  CompanyAutoSignature = 2,
}

struct ContactInfo
{
  1:i32 login_id,
  2:string full_name,
  3:string email_address,
}

service AlertService
{
  bool send(1:AlertType alert_id, 2:UserFlag user_flags, 3:i64 promise_id, 4:map<string,string> extra_params, 5:string from_name),
  bool send_users(1:AlertType alert_id, 2:list<i32> user_ids, 3:i64 promise_id, 4:map<string,string> extra_params, 5:string from_name),
  list<ContactInfo> getContactInfo(1:i32 promise_id, 2:UserFlag user_flags, 3:AlertType alert_id),
}

struct AlertMessage
{
   1:i32 alert_message_id,
   2:i32 alert_id,
   3:string subject,
   4:string body,
   5:i32 hours,
   6:i32 company_id,
   7:string created,
   8:string updated,
}

service AlertMessageService
{
  AlertMessage getAlertMessage(1:i32 alert_id, 2:i32 company_id, 3:bool get_default_on_not_found),
  AlertMessage getDefaultAlertMessage(1:i32 alert_id, 2:i32 company_id),
  i32 getDefaultCompanyID(1:i32 company_id)
}

service PromiseAlertService
{
  bool addPromiseAlert(1:AlertType alert_id, 2:i32 login_id, 3:i32 promise_id)
}

enum PredictedCommentType
{
  Transcript = 1,
  Survey = 2,
}

struct PredictedProbability
{
  1:double neg,
  2:double neutral,
  3:double pos,
}

struct PredictedComment
{
  1:string _id,
  2:string sClassification,
  3:i64 iPhraseSentimentId,
  4:string dtUpdated,
  5:bool bAdjusted,
  6:i32 iCorpusInformationId,
  7:string dtCreated
  8:PredictedProbability probabilities,
  9:i32 iPromiseTranscriptId,
  10:string sPhrase,
  11:i32 iIndustryId,
  12:i32 iRanking,
  13:i32 iPromiseId,
  14:i32 iCreatedByLoginId,
}

service PCService
{
  list<PredictedComment> getPredictedTranscriptComments(1:i32 count, 2:IndustryID industry_id, 3:bool adjusted, 4:i32 login_id),
  list<PredictedComment> getPredictedSurveyComments(1:i32 count, 2:IndustryID industry_id, 3:bool adjusted, 4:i32 login_id),
  PredictedComment getPredictedSurveyComment(1:i32 promise_id),
}

struct ClassifiedCommentSentiment
{
  1:i32 id,
  2:string phrase,
  3:string parsed_comment,
  4:PredictedProbability probability,
  5:string label,
  6:bool adjusted,
  7:i32 ranking,
}

service ClassifyCommentSentiment
{
  ClassifiedCommentSentiment classifyCommentSentiment(1:IndustryID industry_id, 2:string phrase, 3:bool load_adjusted_phrase),
}

// appointment

exception InvalidCompany
{
}

exception DoubleBook
{
}

exception InvalidCustomer
{
}

exception InvalidVehicle
{
}

exception InvalidAssignee
{
}

exception InvalidLogin
{
}

exception InvalidTimeStamp
{
}

struct Appointment
{
  1:i32 company_id,
  2:string first_name,
  3:string last_name,
  4:string mobile_phone,
  5:string home_phone,
  6:string email,
  7:string company_service,
  8:i32 timestamp,
  9:string vehicle_make,
  10:optional i64 appointment_id,
  11:i64 user_id,
}

service AppointmentService
{
  NewAppointment saveAppointment(1:NewAppointment appointment) throws(1:InvalidCompany invalid_company, 2:DoubleBook double_book, 3:InvalidCustomer invalid_customer, 4:InvalidLogin invalid_login, 5:InvalidTimeStamp invalid_time_stamp, 6:InvalidVehicle invalid_vehicle),

  void deleteAppointment(1:i32 appointment_id),

  i32 save(1:Appointment appointment, 2:bool calendar) throws(1:InvalidCompany invalid_company, 2: DoubleBook double_book),

  void deleteAppointments(1:i32 appointment_id),

  list<Appointment> getAppointments(1:i32 user_id, 2:i64 start_time, 3:i64 end_time),

  Appointment getAppointment(1:i32 appointment_id) throws (1:PromiseNotFound notfound),

  list<Appointment> getAppointmentsBySearchTerm(1:string search_term, 2:i32 user_id, 3:i32 company_id),

  string getAssignee(1:i32 user_id),

  i32 getServiceManager(1:i32 company_id),
}

// Vehicle Makes
struct VehicleType
{
  1:string id,
  2:string name,
}

service VehicleMakeService
{
  list<string> getAll(),
  bool useVehicleTypesFromDMS(1:i64 company_id),
  list<VehicleType> getMakesForYear(1:i64 company_id, 2:i32 year),
  list<VehicleType> getModelsForMake(1:i64 company_id, 2:i32 year, 3:string make_id),
}

// Messaging

struct MobileMessage
{
  1:string mobile_phone,
  2:string subject,
  3:string message,
  4:string time_zone,
  5:optional string return_address,
  6:optional bool check_dups = true,
  7:optional CompanySchedule schedule,
}

exception SMSProviderException
{
  1: string message,
}

service MessagingService
{
  i32 getSmsOutboundQueueSize(),
  i32 getEmailOutboundQueueSize(),
  i32 sendMobileMessage(1:MobileMessage message),
  bool myMail(1:string email, 2:string subject, 3:string message, 4:i32 login_id, 5:string from_address, 6:string from_name),
  string orderNumberForUser(1:i32 login_id) throws (1:SMSProviderException exc),
  string orderNumberForCompany(1:i32 company_id) throws (1:SMSProviderException exc),
  string disconnectNumberForUser(1:i32 login_id) throws (1:SMSProviderException exc),
  string disconnectNumberForCompany(1:i32 company_id) throws (1:SMSProviderException exc),
}

exception RequestFailed
{
}

struct ResponseItem
{
  1:string id,
  2:string value
}

// DMS

struct DMS
{
}

struct Customer
{
  1:i32 iCustomerID,
  2:string sCustomerFName,
  3:string sCustomerLName,
  4:string sMobilePhone,
  5:string sEmail,
  6:string sHomePhone,
  7:string sWorkPhone,
  8:string sAddress,
  9:string sGender,
  10:string sState,
  11:string sTitle,
  12:string sZip,
  13:i32 iAPIUserID,
  14:string sExternalID,
  15:string sCountry,
  16:string sCity,
  17:string sAddress1,
  18:string sAddress2,
}

struct Vehicle
{
  1:i32 iVehicleID,
  2:string sYear,
  3:string sMake,
  4:string sModel,
  5:string sTrim,
  6:string sMileage,
  7:string sVin,
  8:string sColor,
  9:i32 iAPIUserID,
  10:string sExternalID,
  11:i32 iMakeID,
  12:i32 iModelID,
  13:i32 iTrimID,
  14:string sLicense,
  16:string iCustomerID,
  17:string sEngine,
}

struct Employee
{
  1:i32 iLoginID,
  2:string sFullName,
  3:string sEmailAddress,
  4:string sIncrement,
  5:i32 iAPIUserID,
  6:string sExternalID,
  7:string iCompanyID,
}

struct ToyotaOperation
{
	1:string sOpCodeNumber,
	2:string sCombinationCode,
	3:string sOpCodeDescription,
	4:string sServiceOpreationsDescription,
	5:string sOpCodeHours,
	6:string sTechnicianFName,
	7:string sTechnicianMName,
	8:string sTechnicianLName,
	9:string sTechnicianID
}

struct Service
{
  1:i32 iServiceID,
  2:string sServiceName,
  3:i32 iAPIUserID,
  4:string sExternalID,
  5:string iCompanyID,
  6:i32 iLaborCost,
  7:string sLaborCostCurrency,
  9:string sCorrections,
  10:string sComplaint,
  11:i32 iPartsCost,
  12:string sOpCode,
  13:string sServiceType,
  14:string sLaborType,
  15:string sConditionNumber,
  16:string sLaborCodeDescription,
  17:string sClaimNumber,
  18:string sAuthenticationNumber,
  19:string sAuthenticationType,
  20:string sTotalConditionHours,
  21:string sConditionFluidCost,
  22:string sConditionSubletCost,
  23:string sConditionShopCost,
  24:string sTotalConditionCost,
  25:string sConditionCategory,
  26:string sConditionCategoryDescription,
  27:string sCombinationCode,
  28:string sOpCodeDescription,
  29:string sOpCodeHours,
  30:string sTechnicianFName,
  31:string sTechnicianMName,
  32:string sTechnicianLName,
  33:string sTechnicianID,
  34:list<ToyotaOperation> aToyotaOperations,
  35:string sDealertrackServiceType,
  36:string sLaborLevel,
}

struct DefferedService
{
  1:i32 iServiceID,
  2:string sServiceName,
  3:i32 iAPIUserID,
  4:string sExternalID,
}

struct VehiclePart
{
  1:i32 iVehiclePartID,
  2:string sPartNumber,
  3:string sExternalID,
  4:double fPartCost,
  5:double fCoreCost,
  6:string sPartDescription,
  7:bool bInStockFlag,
  8:bool bBackOrderFlag,
  9:bool bDiscontinuedFlag,
  10:i32 iAPIUserID,
}

struct VehicleHistory
{
  1:string iRO,
  2:string sServiceName,
  3:string sServiceWriterFName,
  4:string sServiceWriterLName,
  5:i32 sMileage,
  6:string sCompletedDate,
  7:list<Service> aServices,
  8:string sServiceDealerParty,
  9:string sStatus,
  10:string sServiceInterval,
  11:string sServiceComponents,
  12:string sJobOperationID,
  13:string sUnwarrantable,
  14:string sThirtyDayTrip,
  15:string sServiceDealerAddress,
  16:string sOpenDays,
  17:string sServiceWriterMName,
  18:string sServiceWriterID,
  19:string sTotalCost,
  20:string sTotalCustomerPay,
  21:string sTotalWarrantyPay,
  22:string sTotalInternalPay,
  23:string sTotalExtendedWarranty,
  24:string sOilType,
}

struct VehicleRecallService
{
  1:string sOpCode,
  2:string sDescription,
  3:double fLaborHour,
}

struct VehicleRecalls
{
  1:string sCustomerMemo,
  2:string sCampaignNumber,
  3:string sCampaignDescription,
  4:string sCampaignNotes,
  5:string sRepairStatusCode,
  6:string dtReleaseDate,
  7:string dtExpirationDate,
  8:string sMarket,
  9:string sActivityDescription,
  10:string sSafetyRiskDescription,
  11:string sRemedyDescription,
  12:string sRemedyAvailable,
  13:string dtRecallDate,
  14:string sLinkText,
  15:string sLinkUrl,
  16:i32 bCompleted,
  17:string sCompletionDescription,
  18:string sRepairTimeCam,
  19:string sEstimatedReplacementRate,
  20:string sPartsRequired,
  21:bool bWarrantyEligibility,
  22:list<VehicleRecallService> aServices,
  23:string sExternalRecallID,
  24:string dtRecallLastUpdated,
  25:i32 iRecallAge,
  26:string sCampaignType,
  27:bool bStopSale,
  28:bool bDontDrive,
  29:bool bPartsAvailable,
  30:string dtPartsAvailableDate,
  31:string sRiskType,
  32:i32 iRiskRank,
  33:i32 iProfitRank,
  34:i32 iOverallRank,
  35:i32 iLaborDifficulty,
  36:double fLaborMin,
  37:double fLaborMax,
  38:double fReimbursement,
  39:string sChild,
  40:string sIsReviewed
}

struct VehicleWarranty
{
  1:string dtWarrantyStart,
  2:string sName,
  3:string dtExpiration,
  4:i32 iOdometerLimit,
  5:string sOdometerUnit,
  6:string sDescription,
  7:string sEligibleVIN,
  8:list<VehicleHistory> aVehicleHistory,
}

struct VehicleMobileLaneCheckin
{
    1:i32 iVehicleMobileLaneCheckinID,
    2:string sExternalID,
    3:i32 iMileage,
    4:string dtCheckin,
    5:i32 iCompanyID,
    6:string sVehicleMake,
    7:string sVehicleModel,
    8:string sRO,
    9:i32 iLoginID,
    10:string sVIN,
    11:bool bWasVehiclehecked,
    12:i32 iVehicleYear,
    13:bool bWasAppointmentMade,
}

struct VehicleHealthWarning
{
    1:string sCode,
    2:i32 iCondFlag,
    3:string dtTimeTriggered,
    4:string sDescription,
    5:string sIcon,
    6:i32 iMileage,
    7:string sOwnersManual,
    8:string sType,
}

struct VehicleHealthStatus
{
    1:i32 iVehicleID,
    2:i32 iMileage,
    3:string sQtyOfEngOilDesc,
    4:string sQtyOfEngOilIcon,
    5:string dtQtyOfEngOilLastUpdTime,
    6:string sQtyOfEngOilStatus,
    7:string sReplaceEngOilDesc,
    8:string sReplaceEngOilIcon,
    9:string dtReplaceEngOilLastUpdTime,
    10:string sReplaceEngOilStatus,
    11:string sSmartKeyBatDesc,
    12:string sSmartKeyBatIcon,
    13:string sSmartKeyBatLastUpdTime,
    14:string sSmartKeyBatStatus,
    15:string dtWarningLastUpdTime,
    16:list<VehicleHealthWarning> aVehicleHealthWarnings,
}

struct ServiceLocationPushResponse
{
  1:string sExternalRecordId,
  2:string sMessage,
  3:i32 iPushResultCode,
  4:string sPushedRecordKey,
  5:string iServiceLocationId,
}

struct OEMAppointmentData
{
    1:i32 iTotalAptsCount,
    2:i32 iKeptAptsCount,
    3:double iPercentOnlineApts,
    4:i32 iMissedAptsCount,
    5:double iPercentKeptAptsCount,
    6:i32 iOnlineCustAptsCount,
    7:double iPercentMissedAptsCount,
    8:i32 iDealerAptsCount,
    9:string sExternalID,
    10:i32 iCompanyID,
    11:double iPercentpercentDealerApts,
    12:string dtDateSent,
}

struct OEMResponse
{
  1:string status_message,
  2:bool success,
}

struct OEMAssociatedVehicle
{
  1:string vin,
  2:string make,
  3:string model,
  4:i32 year,
  5:string customer_role,
  6:i32 mileage,
}

struct DMSResponse
{
  1:string status_message,
  2:bool success,
}

struct LaborType
{
  1:string labor_type_id,
  2:string description
}

struct LaborTypeResponse
{
  1:string status_message,
  2:bool success,
  3:bool is_enabled,
  4:list<LaborType> labor_types,
}

struct VehicleOwnershipResponse 
{
  1:string cur_customer_external_id,
  2:string cur_customer_first_name,
  3:string cur_customer_last_name,
  4:string existing_customer_external_id,
  5:string existing_customer_first_name,
  6:string existing_customer_last_name
}

struct ServiceType
{
  1:string service_type_id,
  2:string description
}

struct ServiceTypeResponse
{
  1:string status_message,
  2:bool success,
  3:bool is_enabled,
  4:list<ServiceType> service_types,
}

service DMSService
{
  list<Service> pullCompanyServices(1:string company_id, 2:string date) throws(1:RequestFailed requestfailed),
  list<Employee> pullServicePersons(1:string company_id, 2:string employee_type) throws(1:RequestFailed requestfailed),
  list<i32> pullCustomerVehicles(1:string company_id, 2:string customer_id) throws(1:RequestFailed requestfailed),
  list<i32> pullCustomers(1:string company_id, 2:Customer customer_options) throws(1:RequestFailed requestfailed),
  Customer pushCustomer(1:string company_id, 2:Customer customer) throws(1:RequestFailed requestfailed),
  string pushAppointment(1:string company_id, 2:NewAppointment appointment) throws(1:RequestFailed requestfailed),
  Vehicle pushCustomerVehicle(1:string company_id, 2:string customer_id, 3:Vehicle vehicle) throws(1:RequestFailed requestfailed),
  list<Service> pullDeferredJobsByVehicleId(1:string company_id, 2:string vehicle_id) throws(1:RequestFailed requestfailed),
  string pushDeferredJobsByVehicleId(1:string company_id, 2:string vehicle_id, 3:string op_code, 4:string external_id, 5:Service service_description) throws(1:RequestFailed requestfailed),
  list<VehicleHistory> pullVehicleHistory(1:string company_id, 2:string customer_id, 3:string vin, 4:string increment) throws(1:RequestFailed requestfailed),
  list<VehicleRecalls> pullVehicleRecalls(1:string company_id, 2:string vin) throws(1:RequestFailed requestfailed),
  string pushEstimateFromAppointment(1:string company_id, 2:string appt_key) throws(1:RequestFailed requestfailed),
  string pushRepairOrderFromAppointment(1:string company_id, 2:string appt_key) throws(1:RequestFailed requestfailed),
  DMSResponse deleteCustomerVehicle(1:string company_id, 2:i32 customer_id, 3:i32 vehicle_id) throws(1:RequestFailed requestfailed),
  LaborTypeResponse getLaborTypes(1:string company_id),
  ServiceTypeResponse getServiceTypes(1:string company_id),
  list<string> getLaborLevels(1:string company_id),
  list<i32> searchDMSCustomer(1:string company_id, 2:string first_name, 3:string last_name, 4:string phone_number, 5:string vin, 6:string make, 7:string model, 8:string year, 9:string license_plate),
  VehicleOwnershipResponse fetchCurrentVehicleOwnerFromDMS(1:i32 customer_id, 2:string customer_external_id, 3:i32 vehicle_id, 4:string vehicle_external_id),
  string finalRoToInvoice(1:i32 company_id, 2:i32 login_id, 3:i32 promise_id) throws(1:RequestFailed requestfailed),
}

struct VoluntaryProtectionProductLink {
  1:string checkout_link,
  2:string expiration_date,
  3:string error_message
}

struct VPPBenefit {
  1:list<string> benefits,
  2:string benefit_url,
  3:string video_url
}

struct VPPPlanTermOption {
  1:string description,
  2:bool is_surcharge,
  3:double retail_price,
  4:double net_cost,
  5:string code
}

struct VPPPlanTerm {
  1:string plan_name,
  2:i32 term_months,
  3:i32 term_mileage,
  4:i32 term_services,
  5:i32 term_services_limit,
  6:i32 term_services_interval,
  7:string form_id,
  8:string form_name,
  9:double retail_surcharge,
  10:double net_surcharge,
  11:double deductible,
  12:string deductible_type,
  13:double deductible_reduced,
  14:bool is_discountable,
  15:double dealer_cost,
  16:double retail_price,
  17:double retail_price_basis,
  18:double pack_fee,
  19:double tax,
  20:string pricing_rule_type,
  21:list<VPPPlanTermOption> term_options,
  22:list<i32> payment_options
}

struct VPPPlan {
  1:string name,
  2:string code,
  3:list<VPPBenefit> coverage_benefits,
  4:list<VPPPlanTerm> plan_terms,
}

struct VoluntaryProtectionProduct {
  1:i32 product_type_id,
  2:i32 provider_id,
  3:string admin_dealer_code,
  4:list<VPPPlan> plans,
}

service OEMService
{
  list<VehicleWarranty> get_vehicle_warranty(1:string company_id, 2:string vin),
  VehicleMobileLaneCheckin push_vehicle_mobile_lane_checkin(1:string company_id, 2:VehicleMobileLaneCheckin vehicle_mobile_lane_checkin),
  string push_vehicle_mpi(1:string company_id, 2:list<Issue> vehicle_mpi, 3:i32 promise_id, 4:i32 vehicle_id),
  list<VehicleHistory> get_national_service_history(1:string company_id, 2:string vin),
  VehicleRecallResponse get_special_service_campaign(1:string company_id, 2:string vin),
  Vehicle get_vehicle_details(1:string company_id, 2:string vin),
  VehicleHealthStatus get_vehicle_health_status(1:string company_id, 2:i32 vehicle_id),
  OEMResponse push_appointments(1:i32 company_id, 2:OEMAppointmentData appointments),
  list<VehiclePart> search_vehicle_parts(1:i32 company_id, 2:string partial_part_number, 3:string partial_part_description),
  list<Service> search_opcodes(1:i32 company_id, 2:string partial_opcode),
  list<Service> search_opcodes_by_make(1:i32 company_id, 2:string vehicle_make, 3:string partial_opcode),
  VehicleRecallResponse get_special_service_campaign_by_make(1:string company_id, 2:string vin, 3:string vehicle_make),
  list<ProtectionProduct> get_protection_products(1:string company_id, 2:string vin),
  list<OEMAssociatedVehicle> get_customer_associated_vehicles(1:string company_id, 2:string customer_id),
  list<map<string,string>> get_tire_recommendations(1:string company_id, 2:string location_code, 3:string jsonified_param),
  string create_consumer_lead(1:string company_id, 2:string location_code, 3:string jsonified_request_body),
  list<LoyaltyEngagement> get_loyalty_details(1:string company_id, 2:list<string> vins),
  list<VoluntaryProtectionProduct> get_voluntary_protection_products(1:string company_id, 2:i32 promise_id),
  VoluntaryProtectionProductLink get_voluntary_protection_products_link(1:string company_id, 2:i32 promise_id),
}

struct VehicleRecallResponse
{
  1:string error_message,
  2:list<VehicleRecalls> service_campaigns
}

struct LoyaltyEngagement
{
  1:string vin,
  2:string engagement_color,
  3:string home_dealer,
  4:string pma_dealer,
  5:string preferred_dealer,
  6:bool is_customer_impacting,
  7:string vin_response
}

struct ProtectionProductService
{
  1:string description,
  2:string status,
  3:string status_desc,
  4:string opcode,
  5:string servicing_dealer,
  6:string service_date,
  7:i32 mileage
}

struct ProtectionProductLayer
{
  1:string code,
  2:string description,
  3:string detail
}

struct ProtectionProduct
{
  1:string provider,
  2:string type,
  3:string dealer_code,
  4:string vin,
  5:string selling_dealer,
  6:string effective_date,
  7:string expiration_date,
  8:string subtype,
  9:string plan_code,
  10:i32 plan_coverage_months,
  11:i32 expiration_mileage,
  12:string agreement_number,
  13:string agreement_url,
  14:string status_code,
  15:string status_description,
  16:string form_number,
  17:i32 form_month,
  18:i32 form_year,
  19:string deductible,
  20:string deductible_type,
  21:list<ProtectionProductService> service_info,
  22:ProtectionProductLayer product_category,
  23:ProtectionProductLayer product_type,
  24:ProtectionProductLayer product_plan,
  25:ProtectionProductLayer product_subplan
}

struct PromisePart
{
1:i32 promise_part_id,
2:i32 promise_id,
3:i32 company_part_id,
4:i32 quantity_ordered,
5:i32 quantity_received,
6:i32 quantity_pending_receipt,
7:string date_expected,
8:string date_received,
9:double cost,
10:string part_number,
11:string part_make,
12:string description,
}

struct Order{
1:i32 promise_id,
2:string unique_identifier,
3:double customer_amount,
4:string date_in,
5:string date_promised,
6:string date_completed,
7:string date_delivered,
8:i32 customer_id,
9:Customer customer,
10:i32 vehicle_id,
11:Vehicle vehicle,
12:list<PromisePart> parts,
13:i32 service_advisor_id,
14:Employee service_advisor,
}

service OrderService
{
list<PromisePart> get_promise_parts(1:i32 promise_id),
list<Order> get_orders(1:list<i32> company_ids, 2:string start_date, 3:string end_date),
}

// MotorsAPI

struct Motors
{
}

struct RSServicePackage
{
  1:string group_id,
  2:string group_name,

  3:string taxonomy_id,
  4:string taxonomy_name,

  5:string sg_id,
  6:string sg_name,

  7:string system_id,
  8:string system_name,

  9:string qualifier_id,
  10:string qualifier_description
}

struct MotorsModels
{
  1:string make,
  2:list<string> models
}

service MotorsService
{
  list<map<string, string>> requestMotorsYears() throws(1:RequestFailed requestfailed),
  list<map<string, string>> requestMotorsMakes(1:string year) throws(1:RequestFailed requestfailed),
  list<map<string, string>> requestMotorsModels(1:Vehicle vehicle) throws(1:RequestFailed requestfailed),
  list<map<string, string>> requestMotorsTrims(1:Vehicle vehicle) throws(1:RequestFailed requestfailed),
  string requestMotorsVehicleImage(1:Vehicle vehicle) throws(1:RequestFailed requestfailed),
  list<RSServicePackage> requestMotorsRecommendedServices(1:Vehicle vehicle) throws(1:RequestFailed requestfailed),
  list<Vehicle> requestMotorsVehicleByVIN(1:string vin) throws(1:RequestFailed requestfailed),
  list<MotorsModels> requestAllMotorsMakeModels() throws(1:RequestFailed requestfailed),
}

// Industry

struct Industry
{
  1:i32 iIndustryID,
  2:string sDisplayName,
  3:string sSystemName,
  4:string sServiceName,
  5:string sWelcomeEmail,
  6:string sPasswordEmail,
  7:string sShortCode,
  8:i32 iOptOutNotifyEmail,
  9:i32 iOptOutCommPrefNotify,
  10:i32 iAutoArchiveDays,
  11:i32 iAutoCancelDays,
  12:i32 iCSIDelayDays,
  13:i32 iCSIFirstAlertDays,
  14:i32 iCSISecondAlertDays,
  15:string sSurveyURL,
  16:CompanySchedule oSMSDeliverySchedule,
  17:i32 iAllowCancelPromise,
  18:string sMailSender,
  19:string sGatingScript,
  20:string sDefaultManagerName,
}

exception IndustryNotFound
{
}

service IndustryService
{
  Industry getById(1:i32 industry_id) throws(1:IndustryNotFound notfound),
  void save(1:Industry industry),
}

// System

service SystemService
{
  string getValue(1:string key),
  void setValue(1:string key, 2:string value),
  void ping(),
  string test_echo(1:string test),
}

// VehicleService
enum VehicleServiceSource
{
  Appointment = 1,
  Vci = 2,
  Mpi = 3,
}

// Transcripts

enum TranscriptMessageType
{
  Status = 1,
  CustomerReply = 2,
  CustomerOptOutReply = 3,
  EstimatorReply = 4,
  InsuranceReply = 5,
  RepairAuthRequest = 6,
  RepairAuthReply = 7,
  PostDeliverySurveyRequest = 8,
  ShopSurveyRequest = 9,
  InsuranceSurveyRequest = 10,
  ListenInvitation = 11,
  CustomerOptInReply = 12,
}

enum TranscriptSource
{
  Shop = 1,
  Customer = 2,
  Estimator = 3,
  Insurance = 4,
}

enum TranscriptSourceType
{
  Email = 1,
  SMS = 2,
  Manual = 3,
  System = 4,
  Web = 5,
}

enum TranscriptSourceComm
{
  System = 1,
  Manual = 2,
}

enum TranscriptDest
{
  Customer = 1,
  Estimator = 2,
  Insurance = 3,
}

enum TranscriptDestType
{
  Email = 1,
  SMS = 2,
  Web = 3,
}

enum ApiFeedType
{
  Out = 3,
}

const string TRANSCRIPT_SOURCE_COMM_SYSTEM = "updatepromise.com"
const string TRANSCRIPT_SOURCE_COMM_MANUAL = ""

struct Transcript
{
  1:i32 iPromiseID,
  2:string dtDateTimeStamp,
  3:TranscriptMessageType sMessageType,
  4:TranscriptSource sSource,
  5:TranscriptSourceType sSourceType,
  6:string sSourceComm,
  7:TranscriptDest sDest,
  8:TranscriptDestType sDestType,
  9:string sDestComm,
  10:string sSubject,
  11:string sBody,
  12:string sBodyFiltered,
  13:string sBodyHTML,
  14:i32 iSMSCount,
  15:string sExternalID,
  16:i32 iInsuranceCompanyID,
  17:i32 iPresetMessageID,
}

service TranscriptService
{
  i32 save(1:Transcript transcript, 2:i32 api_user_id),
}

// Post-Delivery Feedback

service PostFeedbackService
{
  bool sendInvite(1:i32 promise_id),
}

// CSI

service CSIService
{
  bool sendInvite(1:i32 promise_id),
  bool publishCSI(1:i32 log_event_id),
}

// TCPA Replies Service

enum TcpaReplyStatus
{
  UNREVIEWED = 0,
  REVIEWED = 1,
  STOP_ADDED = 2,
  REVIEWED_ADDED = 3,
}

struct TcpaReply
{
  1:i32 reply_id,
  2:i64 timestamp,
  3:TcpaReplyStatus status,
  4:string body,
  5:i32 promise_id,
}

enum TcpaPhraseType
{
  STOP = 1,
  REVIEWED = 2,
}

struct TcpaPhrase
{
  1:i32 phrase_id,
  2:TcpaPhraseType phrase_type,
  3:string phrase,
  4:i64 date_created,
}

service TcpaService
{
  list<TcpaReply> getReplies(1:i32 user_id, 2:list<TcpaReplyStatus> statuses,
                             3:i32 row_start, 4:i32 rows_per_page),
  i32 getRepliesCount(1:i32 user_id, 2:list<TcpaReplyStatus> statuses),
  void markReviewed(1:i32 reply_id),
  void reviewedAndAdd(1:i32 reply_id),
  void stopAndAdd(1:i32 reply_id),
  void insertReply(1:i32 transcript_id),

  list<TcpaPhrase> getPhrases(1:list<TcpaPhraseType> types,
                              3:i32 row_start, 4:i32 rows_per_page, 5:string search),
  void removePhrase(1:i32 tcpa_phrase_id),
  i32 addPhrase(1:TcpaPhraseType phrase_type, 2:string phrase),
  bool checkStopPhrase(1:string phrase),
}

// Payments

enum PaymentDepartment
{
  Main = 1
  Service = 2
  Parts = 3
  Finance = 4
}

enum PaymentMode
{
  Terminal = 1, // Terminal/EMV/Contactless
  Keyed = 2,
  Mobile = 3, // Glovebox
  Swiped = 4, // Magnetic card reader
}

enum PaymentMethod
{
  Cash = 1
  Credit = 2,
  Debit = 3,
  Check = 4,
  Finance = 5,
}

enum CardNetwork
{
  Visa = 1,
  MasterCard = 2,
  Discover = 3,
  AmericanExpress = 4,
  Maestro = 5,
  ChinaUnionPay = 6,
}

enum PaymentErrorCode
{
  ErrorUnknown = 1,
  InvalidCreditCardNumber = 2,
  InvalidExpirationMonth = 3,
  InvalidExpirationYear = 4,
  InvalidCardType = 5,
  InvalidAmount = 6,
  InvalidTransactionID = 7,
  AuthorizationExpired = 8,
  InvalidBillingName = 9,
  InvalidZip = 10,
  InvalidCSC = 11,
  TransactionNotAuthorized = 12,
  TransactionPendingSettlement = 13,
  SignatureError = 14,
  DuplicateTransaction = 15, // Same transaction too soon
  CardTypeNotSupported = 16,
  HostError = 17,
  PartialTransactionNotSupported = 18,
  ExpiredCard = 19,
  TerminalTimeout = 20,
  CallIssuer = 21,
  TerminalBusy = 22,
  RetryTransaction = 23,
  InvalidCardMethod = 24,
  BlockedPotentialFraud = 25,
  InvalidRefundAmount = 26,
  TerminalNotConnected = 27,
  InsufficientFunds = 28,
  TransactionCanceled = 29,
  InvalidMerchantIdentifier = 30,
  InvalidSessionIdentifier = 31,
  InvalidCharacter = 32,
  InvalidVoidAmount = 33,
}

struct PaymentAuthorizationRequest
{
  1:string number,
  2:i32 expiration_month,
  3:i32 expiration_year,
  4:string csc,
  5:string name,
  6:string zip,
}

struct PaymentResponse
{
  1:bool success,
  2:map<PaymentErrorCode, list<string>> errors,
  3:i32 transaction_id,
  4:double change_due,
}

struct SignatureResponse
{
  1:bool success,
  2:string error,
  3:string signature, // base64 encoded png
}

enum PaymentType
{
  JOB = 1,
  COUNTER = 2,
}

enum PaymentStatus
{
  Unpaid = 1,
  PartiallyPaid = 2,
  Paid = 3,
  Canceled = 4,
}

enum TransactionType
{
  Sale = 1,
  Void = 2,
  Refund = 3,
}

enum TransactionStatus
{
  Authorized = 1,
  Completed = 2,
  Declined = 3,
  Voided = 4,
}

struct PaymentInfo
{
  1:string customer_name,
  2:string employee_name,
  3:string contact_method,
  4:string contact_info,
  5:string company_name,
  6:string company_logo,
  7:string company_phone,
  8:double tax,
  9:double total,
  10:list<map<string,string>> items,
  11:PaymentStatus status,
  12:i32 attachment_id,
  13:PaymentGateway gateway,
  14:string transaction_id,
  15:string external_order_id,
  16:string receipt_json,
  17:string company_address,
  18:string company_city,
  19:string company_state,
  20:string company_zip,
  21:string company_timezone,
  22:string company_url,
  23:bool is_sale,
  24:bool is_embedded,
  25:bool is_tokenized,
  26:i32 api_user_id
  27:string external_job_id,
  28:double balance_due,
  29:string notes,
  30:bool has_transactions,
  31:PaymentDepartment department,

  100:string vehicle_make,
  101:string vehicle_model,
  102:string vehicle_year,
}

struct EMVTag
{
  1:string key,
  2:string value,
}

struct EMVReceipt
{
  1:string application_id,
  2:string application_name,
  3:string cryptogram,
  4:list<EMVTag> tags,
}

struct PaymentReceipt
{
  1:double total,
  2:string name,
  3:string date,
  4:string transaction_id,
  5:string masked_number,
  6:string card_logo,
  7:string transaction_type,
  8:string entry_mode,
  9:string response_code,
  10:string response_message,
  11:string merchant_id,
  12:string approval_code,
  13:string terminal_id,
  14:string reference_number,
  15:bool pin_verified,
  16:EMVReceipt emv,
  17:i32 payment_id,
  18:TransactionStatus status,
  19:string date_voided,
  20:PaymentMethod payment_method,
  21:i32 check_number,
  22:double surcharge,
}

struct PaymentHistory
{
  1:i64 id,
  2:i64 promise_id,
  3:string company_name,
  4:string employee_name,
  5:string first_name,
  6:string last_name,
  7:string transaction_id,
  8:i32 status,
  9:double tax,
 10:double total,
 11:list<map<string,string>> items,
 12:i64 created,
 13:i64 viewed,
 14:i64 canceled,
 15:i64 authorized,
 16:i64 captured,
 17:i64 voided,
 18:i32 attachment_id,
 19:PaymentGateway gateway,
 20:string external_order_id,
 21:string vehicle_make,
 22:string vehicle_model,
 23:i32 vehicle_year,
 24:string external_job_id,
 25:i32 api_user_id,
 26:string customer_phone,
 27:string customer_email,
 28:string vin,
 29:double balance_due,
 30:string notes,
 31:bool has_transactions,
 32:PaymentDepartment department,
}

struct PaymentTransaction
{
  1:i32 payment_transaction_id,
  2:CardNetwork card_network,
  3:string last_four,
  4:double amount,
  5:string date_created,
  6:PaymentGateway gateway,
  7:TransactionStatus status,
  8:TransactionType transaction_type,
  9:PaymentMethod payment_method,
  10:string transaction_id,
  11:string external_order_id,
  12:PaymentDepartment department_id,
}

struct PaymentRequest
{
  1:i32 login_id,
  2:i32 promise_id,
  3:string first_name,
  4:string last_name,
  5:i32 api_user_id,
  6:string external_job_id,
  7:double tax,
  8:double total,
  9:map<string, double> items,
 10:i32 attachment_id,
 11:PaymentType type,
 12:bool is_manual,
 13:bool require_signature,
 14:PaymentOrderNumberType order_number_type,
 15:string order_number,
 16:string order_date,
 17:string notes,
 18:PaymentDepartment department,
}

struct PaymentTerminal
{
  1:string terminal_id,
  2:string name,
  3:string description,
  4:string serial,
  5:string model,
}

enum PaymentGateway
{
  PayTrace = 1,
  Clover = 2,
  Vantiv = 3,
  CardConnect = 4,
  PayJunction = 5,
  Chase = 6,
}

enum PaymentOrderNumberType
{
  SpecialOrderPartNumber = 1,
  WarrantyNumber = 2,
  DealNumber = 3,
}

struct PaymentSearch
{
  1:PaymentStatus status,
  2:list<i32> companies,
  3:string date_from,
  4:string date_to,
  5:string date_filter,
  6:string external_job_id,
  7:string customer_name,
  8:string employee_name,
  9:string vin,
  10:string company_name,
}

struct PaymentSearchResult
{
  1:i32 count,
  2:list<PaymentHistory> payments,
}

struct PaymentDeviceMapping
{
  1:i32 id,
  2:i32 company_id,
  3:string serial,
  4:string name,
}

exception PaymentInfoInvalid
{
}

exception PaymentReceiptException
{
}

exception PaymentImportException
{
  1:string message,
}

exception PaymentTicketNotFound
{
  1:string message,
}

service PaymentService
{
  PaymentResponse authorize(1:i32 payment_id, 2:double amount, 3:PaymentAuthorizationRequest auth_request,
                            4:i32 login_id, 5:PaymentDepartment department),
  PaymentResponse capture(1:i32 transaction_id, 2:string signature, 3:map<string, string> params),
  PaymentResponse swiped_sale(1:i32 payment_id, 2:double amount, 3:PaymentAuthorizationRequest auth_request,
                              4:string signature, 5:i32 login_id, 6:PaymentDepartment department),
  PaymentResponse sale(1:i32 payment_id, 2:double amount, 3:PaymentAuthorizationRequest auth_request, 4:string signature,
                       5:i32 login_id, 6:map<string, string> params, 7:PaymentDepartment department),
  string embedded_authorization(1:i32 payment_id, 2:double amount, 3:map<string, string> params, 4:PaymentDepartment department),
  string embedded_sale(1:i32 payment_id, 2:double amount, 3:map<string, string> params, 4:PaymentDepartment department),
  PaymentResponse voidorize(1:i32 transaction_id, 2:i32 login_id, 3:map<string, string> params),
  PaymentResponse refund(1:i32 transaction_id, 2:double amount, 3:i32 login_id, 4:map<string, string> params),
  PaymentResponse terminal_sale(1:i32 payment_id, 2: double amount, 3:i32 login_id,  4:map<string, string> params, 5:PaymentDepartment department),
  PaymentResponse process_embedded_authorization(1:i32 payment_id, 2:string processor_response, 3:i32 login_id, 4:PaymentDepartment department),
  PaymentResponse process_embedded_sale(1:i32 payment_id, 2:string processor_response, 3:i32 login_id, 4:PaymentDepartment department),
  PaymentResponse cash_sale(1:i32 payment_id, 2:double amount, 3:i32 login_id, 4:PaymentDepartment department),
  PaymentResponse check_sale(1:i32 payment_id, 2:double amount, 3:i32 check_number, 4:i32 login_id, 5:PaymentDepartment department),
  PaymentResponse finance(1:i32 payment_id, 2: double amount, 3:i32 login_id, 4:PaymentDepartment department, 5:string transaction_id),
  SignatureResponse payment_signature(1:i32 payment_id, 2:PaymentDepartment department, 3:string terminal_id, 4:map<string, string> params),
  SignatureResponse promise_signature(1:i32 promise_id, 2:PaymentDepartment department, 3:string terminal_id, 4:map<string, string> params),
  SignatureResponse company_signature(1:i32 company_id, 2:PaymentDepartment department, 3:string terminal_id, 4:map<string, string> params),
  bool assignPromiseToPayment(1:i32 payment_id, 2:i32 promise_id),
  bool view(1:i32 payment_id),
  bool cancel(1:i32 payment_id),
  PaymentInfo getPaymentInfo(1:i32 payment_id) throws (1:PaymentInfoInvalid invalid),
  PaymentReceipt getPaymentReceipt(1:i32 transaction_id) throws (1:PaymentReceiptException error),
  string tokenized_field(1:i32 payment_id),
  binary getReceiptPDF(1:i32 transaction_id),
  i32 createPayment(1:PaymentRequest payment, 2:string invitation_message),
  list<PaymentHistory> getPaymentHistory(1:i32 promise_id, 2:bool promise_completed),
  list<PaymentTransaction> getPaymentTransactions(1:i32 payment_id),
  PaymentSearchResult searchPayments(1:i32 login_id, 2:PaymentSearch search, 3:i32 row_start, 4:i32 rows_per_page),
  bool paymentUserAccess(1:i32 payment_id, 2:i32 login_id),
  void importPayment(1:i32 login_id, 2:PaymentType payment_type, 3:string external_job_id)
                     throws(1:PaymentTicketNotFound notfound, 2:PaymentImportException error),
  void pushPayment(1:i32 payment_id),
  PaymentGateway getPaymentGateway(1:i32 payment_id),
  list<PaymentTerminal> getTerminals(1:i32 payment_id, 2:PaymentDepartment department),
  list<PaymentTerminal> getTerminalsByCompanyID(1:i32 company_id, 2:PaymentDepartment department),
  PaymentResponse saveSignature(1:i32 payment_id, 2:string signature, 3:bool is_mpi_signature),
  void pullPaymentAmount(1:i32 payment_id),
  i32 getReferenceNumber(1:i32 payment_id),
  list<Company> getCompanies(1:i32 login_id),
  map<string, string> getCredentials(1:i32 company_id),
  string logPaymentAttempt(1:double amount, 2:string last_four, 3:i32 company_id, 4:i32 login_id),
  bool checkDuplicatePayment(1:double amount, 2:string last_four, 3:i32 company_id),
  list<PaymentDepartment> getDepartments(1:i32 company_id),
  list<PaymentDeviceMapping> getPaymentDeviceMappings(1:i32 company_id),
  i32 addPaymentDeviceMapping(1:PaymentDeviceMapping mapping),
  void deletePaymentDeviceMapping(1:i32 mapping_id),
}

// Short URLs

service TinyUDPService
{
  i32 decode_promise_path(1:string path),
  string encode_promise_url(1:i32 promise_id),
}

// Profiles

struct Profile
{
  1:string customer_name,
  2:string contact_method,
  3:string email,
  4:string mobile_phone,
  5:string home_phone,
  6:string work_phone,
  7:string unique_identifier,
  8:string vehicle_make,
  9:string vehicle_model,
  10:i32 vehicle_year,
  11:string company_name,
  12:string company_logo,
  13:string company_phone,
  14:string company_timezone,
  15:i32 company_id,
  16:string branding,
  17:i32 customer_id
}

struct ProfileEvent
{
  1:string date_event,
  2:string subject,
  3:string description,
}

exception ProfileNotFound
{
}

service ProfileService
{
  bool hasConsent(1:i32 promise_id),
  void setConsent(1:i32 promise_id, 2:bool consent, 3:string ip_address, 4:string user_agent),
  Profile getByPromiseID(1:i32 promise_id) throws (1:ProfileNotFound notfound),
  bool validate(1:i32 promise_id, 2:string vehicle_make, 3:string contact_info),
  list<ProfileEvent> getHistory(1:i32 promise_id),
  bool validateByContactInfoOnly(1:i32 promise_id, 2:string contact_info),
}

// Electronic Signatures

enum DocumentStatus
{
  Created = 1,
  Sent = 2,
  Viewed = 3,
  Signed = 4,
}

enum DocumentSignatureType
{
  Signature = 1,
  Initials = 2,
}

struct DocumentSignatureArea
{
  1:DocumentSignatureType type,
  2:i32 page,
  3:i32 width,
  4:i32 height,
  5:i32 top,
  6:i32 left,
  7:string signature,
}

struct DocumentImage
{
  1:i32 width,
  2:i32 height,
  3:string data,
}

struct Document
{
  1:i32 document_id,
  2:i32 promise_id,
  3:string filename,
  4:string date_created,
  5:string date_sent,
  6:string mime_type,
  7:DocumentStatus status,
  8:list<DocumentSignatureArea> signature_areas,
  9:string file_hash,
  10:string file_url,
  11:string thumb_url,
  12:i32 api_user_id,
  13:string s3_url,
  14:bool converted_flag,
  15:string vtt,
}

struct IssueCustomerApproval
{
  1:i32 issue_id,
  2:bool customer_approved,
  3:string approval_method,
}

enum PartsStatus
{
  InStock = 1,
  OutOfStock = 2,
  Backorder = 3,
}

struct Issue
{
 1:i32 issue_id,
 2:string employee_notes,
 3:string correction,
 4:string parts,
 5:double employee_amount,
 6:double parts_price,
 7:double labor_price,
 8:double labor_unit,
 9:string op_code,
 10:string technician_notes,
 11:bool needs_approval,
 12:bool customer_approved,
 13:i32 service_id,
 14:list<Document> documents,
 15:string key,
 16:PartsStatus parts_status,
 17:string parts_backorder_date,
 18:string labor_type,
 19:string severity,
 20:string approval_method,
 21:i32 sort_order,
 23:double shop_supply_fee,
 24:bool send_to_estimate,
 25:double discount,
 26:string discount_type,
 27:i32 labor_id,
 28:bool is_dms_issue,
 29:i32 technician_id,
 30:string dispatch_code,
 31:string est_duration,
 32:i32 parts_person_id,
 33:i32 vehicle_service_id
}

struct IssueApproval
{
  1:i32 approval_id,
  2:list<Issue> issues,
  3:string customer_notes,
  4:double total,
}

struct ListIssueApproval
{
  1:i32 approval_id,
  2:i64 date_approved,
}

struct DocumentEvent
{
  1:string action,
  2:string ip_address,
  3:string user_agent,
  4:string url,
}

exception DocumentNotFound
{
}

exception DocumentImportException
{
  1:string message,
}

service DocumentService
{
  list<Document> getDocuments(1:i32 promise_id),
  Document getDocument(1:i32 document_id) throws (1:DocumentNotFound notfound),
  PaymentResponse signDocument(1:i32 document_id, 2:list<DocumentSignatureArea> signature_areas, 3:DocumentEvent event),
  binary getDocumentData(1:i32 document_id, 2:bool signed, 3:DocumentEvent event) throws (1:DocumentNotFound notfound),
  string getDocumentUrl(1:i32 document_id, 2:bool signed, 3:DocumentEvent event) throws (1:DocumentNotFound notfound),
  void logDocumentEvent(1:i32 document_id, 2:DocumentEvent event),
  list<DocumentImage> getDocumentImages(1:i32 document_id,),
  void saveSignatureAreas(1:i32 document_id, 2:list<DocumentSignatureArea> signature_areas),
  Document createDocument(1:i32 promise_id, 2:string filename, 3:string file_hash,
                          4:list<DocumentSignatureArea> signature_areas, 5:list<string> page_hashes,
                          6:i32 user_id, 7:i32 issue_id),
  bool deleteDocument(1:i32 document_id),
  bool softDeleteDocument(1:i32 document_id),
  i32 updateCustomerApproval(1:i32 promise_id, 2:list<IssueCustomerApproval> approvals, 3:string message, 4:bool from_tech, 5:double ssf),
  i32 updateApprovalSetting(1:i32 document_id, 2:bool needs_approval, 3:string notes, 4:double price) throws (1:DocumentNotFound notfound),
  list<Issue> getUnapproved(1:i32 promise_id),
  IssueApproval getApproval(1:i32 approval_id),
  list<ListIssueApproval> getApprovals(1:i32 promise_id),
  string upload(1:binary data, 2:string key, 3:string filename),
  Document importRoPdf(1:i32 promise_id) throws (1:DocumentNotFound notfound),
  string getVideoUrl(1:string path),
  string getAssetByPath(1:string path, 2:string bucket),
}

// MPI

struct MPIPromiseInfo
{
  1:string unique_id,
  2:string location_id,
  3:i32 company_id,
}

struct ASGOLaborLevel
{
  1:i32 labor_level_id,
  2:string labor_level_name,
  3:string labor_level_description,
  4:bool is_active,
  5:bool is_default,
}

// Used for saveServiceIssueLaborLevel
enum ServiceIssueItemType {
  VehicleServiceItem = 1,
  PromiseIssueItem = 2, 
}

//labor level search/insertion/update results
enum GOResponse {
  INSERT_SUCCESS = 1,
  INSERT_ERROR = 2,
  ROW_EXISTS = 3,
  SEARCH_ERROR = 4,
  INVALID_SERVICE_TYPE = 5,
  UPDATE_SUCCESS = 6,
  UPDATE_ERROR = 7,
  NO_SUCH_COMPANY = 8,
}

service MPIService
{
  Issue getIssue(1:i32 issue_id),
  list<Issue> getIssues(1:i32 promise_id),
  bool deleteIssue(1:i32 issue_id),
  i32 updateIssue(1:i32 issue_id, 2:i32 service_id, 3:i32 promise_id, 4:string issue,
                  5:string correction, 6:string parts, 7:double parts_price, 8:double labor_price, 9:double labor_unit,
                  10:string op_code, 11:string notes, 12:bool needs_approval, 13:string key,
                  14:i32 parts_status, 15:string parts_backorder_date, 16:string labor_type, 17:string severity,
                  18:i32 sort_order, 19:double estimated_job_total, 20:double shop_supply_fee,
                  21:bool send_to_estimate, 22:string approval_method, 23:bool approved, 24:double discount,
                  25:string discount_type, 26:bool is_dms_issue, 27:i32 technician_id, 28:string dispatch_code,
                  29:string est_duration, 30:i32 parts_person_id, 31:i32 vehicle_service_id),
  Document createDocument(1:i32 issue_id, 2:string filename, 3:string file_hash, 4:i32 user_id),
  void send(1:i32 promise_id),
  void save(1:i32 promise_id),
  i32 getPromiseId(1:i32 company_id, 2:string unique_id) throws (1:PromiseNotFound notfound),
  void markViewed(1:i32 promise_id),
  string getForm(1:i64 promise_id),
  void getPartsAvailability(1:i32 promise_id, 2:list<string> part_numbers, 3:optional string part_sale_type),
  bool updateTechnician(1:i32 promise_id, 2:i32 login_id),
  // AutoSoft GO support
  list<ASGOLaborLevel> getLaborLevelsForDealer(1:i32 company_id),
  void setDefaultLaborLevel(1:i32 labor_level_id),
  i32 saveServiceIssueLaborLevel(1:i32 promise_id, 2:i32 service_issue_id, 3:i32 labor_level_id, 4:i32 item_type),
  bool dealerIsGO(1:i32 company_id),
  i32 addGOCompany(1:i32 company_id, 2:bool active),
  void setCompanyInactive(1:i32 company_id),
}

// Campaigns

enum CampaignDeliveryMethod
{
  TEXT = 1,
  EMAIL = 2,
  TEXT_OR_EMAIL = 3,
  TEXT_AND_EMAIL = 4,
}

struct CampaignItem
{
  1:string customer_fname,
  2:string customer_lname,
  3:string mobile_phone,
  4:string email,
  5:string vehicle_make,
  6:string vehicle_model,
  7:string vehicle_year,
  8:string vehicle_vin,
  9:i64 created_by_user_id,
  10:string created_dt,
}

enum CampaignState
{
    DRAFT = 1,
    WAITING = 2,
    DECLINED = 3,
    APPROVED = 4,
    SCHEDULED = 5,
    SENT = 6,
}

struct Campaign
{
  1:string title,
  2:string subject_text,
  3:string subject_html,
  4:string body_text,
  5:string body_html,
  6:CampaignDeliveryMethod method,
  7:string approval_notes,
  9:string approved_by_user,
  10:string approved_dt,
  11:bool is_template,
  12:string scheduled_dt,
  13:list<CampaignItem> items,
  14:CampaignState state,
  15:i64 assign_to_user_id,
  16:string uploaded_by_user,
  17:string uploaded_dt,
  18:i64 company_id,
  19:string created_by_user,
  20:string scheduled_by_user,
}

struct CampaignTemplate
{
  1:string title,
  2:string subject_text,
  3:string subject_html,
  4:string body_text,
  5:string body_html,
  6:CampaignDeliveryMethod method
}

exception CampaignNotFound
{
}

struct ListCampaign
{
    1:i64 campaign_id,
    2:string title,
    3:string user_fullname,
    4:CampaignDeliveryMethod method
    5:i32 recipient_count,
    6:i32 sent_count,
    7:i32 response_count,
    8:i32 converted_count,
    9:string sent_dt,
   10:string scheduled_dt,
   11:string modified_dt,
}

service CampaignService
{
    void submit(1:i64 compaign_id) throws (1:CampaignNotFound notfound),
    void clearCustomerList(1:i64 campaign_id),
    void addListCustomer(1:i64 campaign_id, 2:string first_name, 3:string last_name,
                         4:string mobile_phone, 5:string email, 6:string vehicle_vin,
                         7:string vehicle_make, 8:string vehicle_model, 9:string vehicle_year,
                         10:i64 user_id),
    void importList(1:i64 campaign_id, 2:string csv_path) throws (1:CampaignNotFound notfound),
    void approve(1:i64 campaign_id, 2:i64 approved_by_user_id, 3:bool approved, 4:string notes) throws (1:CampaignNotFound notfound),
    i64 save(1:i64 campaign_id, 2:i64 company_id, 3:i64 created_by_user_id, 4:i64 assign_to_user_id,
             5:string title, 6:string subject_text, 7:string subject_html, 8:string body_text, 9:string body_html,
             10:CampaignDeliveryMethod method, 11:bool is_template) throws (1:CampaignNotFound notfound),
    void schedule(1:i64 campaign_id, 2:string delivery_dt, 3:bool express_consent, 4:i64 scheduled_by_user_id) throws (1:CampaignNotFound notfound),
    void cancel(1:i64 campaign_id),
    Campaign getById(1:i64 campaign_id) throws (1:CampaignNotFound notfound),
    bool userAccess(1:i64 campaign_id, 2:i64 user_id),
    list<ListCampaign> getCampaigns(1:i64 company_id, 2:CampaignState state),
    list<CampaignTemplate> getTemplates(1:i64 company_id),
}

// PDF Conversion

service PDFConverterService
{
  binary htmlToPDF(1:string input_html),
}

struct PromiseCommunicationResponse{
  1:bool success,
  2:string message,
  3:i64 promise_communication_id
}
service PromiseCommunicationService
{
  PromiseCommunicationResponse save_call_id(1:i64 promise_id, 2:string call_id, 3:i64 login_id)
}

// Dealer Printers

struct Printer{
  1:i32 iCompanyPrinterID,
  2:i32 iCompanyID,
  3:string sDescription,
  4:string sWorkorderPrinter,
  5:string sPartsPrinter,
  6:i32 iWorkorderCopies,
  7:i32 iSortOrder,
  8:bool bDefault,
  9:bool bActive,
 10:bool bShow
}

service PrinterService
{
  i32 updatePrinter(1:i32 printer_id, 2:i32 company_id, 3:string description,
                    4:string workorder_printer, 5:string parts_printer, 6:i32 workorder_copies,
                    7:i32 sort_order, 8:bool default_printer, 9:bool active, 10:bool show),
  Printer getPrinter(1:i32 printer_id),
  list<Printer> getPrinters(1:i32 company_id, 2:bool active),
  bool userAccess(1:i32 printer_id, 2:i32 user_id),
  void setSelectedPrinter(1:i32 user_id, 2:i32 printer_id),
  i32 getSelectedPrinter(1:i32 user_id),
}



// HondaOEM

struct PASError
{
  1:string AdditionalInfo,
  2:string ErrorCode,
  3:string ErrorType,
  4:string Message,
}

struct PartsOrderStatusRequest
{
  1:i32 VendorID,
}



struct PartsOrderRequest
{
  1:i32 VendorID,
  2:i32 OrderAction,
  3:string VIN,
  4:string DealerID,
  5:string ConfirmationNumber,
  6:string AppointmentTimestamp,
}

struct PartsOrderInfo_PartsResponse
{
  1:string ShippingNumber,
  2:string PartNumber,
  3:string PartDescription,
  4:i32 PartQuantity,
  5:string PartStatusCode,
}

struct PartsOrderInfo_Response
{
  1:string ConfirmationNumber,
  2:string SABCode,
  3:string ServiceBulletinNumber,
  4:string ServiceBulletinLnk,
  5:string OrderReferenceNumber,
  6:string LaborOperationCode,
  7:i32 FlatRateTime,
  8:string SABLnk,
  9:list<PartsOrderInfo_PartsResponse> PartsOrderInfo_PartsResponse,
  10:list<PASError> Errors,
}

struct PartsETARecallRequest
{
  1:string DealerID,
  2:string ServiceBulletinNumber,
  3:string VIN,
  4:i32 VendorID,
}

struct PartsETADTCRequest
{
  1:string DealerID,
  2:string SABCode,
  3:string VIN,
  4:i32 VendorID,
}


struct PartsETAResponse
{
  1:string DealerID,
  2:double FlatRateTime,
  3:string PartsETA,
  4:string VIN,
  5:i32 VendorID,
  6:list<PASError> Errors,
}

struct PartsOrderStatusResponse
{
  1:list<PartsOrderInfo_Response> PartsOrderStatusResponse,
  2:list<PASError> Errors,
}

service HondaPASService
{
  PartsETAResponse getPartsETA(1:list<PartsETARecallRequest> partsETARecallRequests
                               2:list<PartsETADTCRequest> partsETADTCRequests)
  PartsOrderInfo_Response getPartsOrder(1:PartsOrderRequest partsOrderRequest)
  PartsOrderStatusResponse getPartsStatus(1:PartsOrderStatusRequest partsOrderStatusRequest)
}

struct HondaRecallsRequest
{
  1:string VIN,
}

struct HondaRecallCdValue
{
  1:string cd,
  2:string value,
}

struct HondaRecallCampaign
{
  1:HondaRecallCdValue AgencyType,
  2:string CampaignBulletinID,
  3:string CampaignDescription,
  4:string CampaignDispositionStatusCode,
  5:string CampaignDispositionStatusDesc,
  6:string CampaignID,
  7:string CampaignStartDate,
  8:string CampaignTypeCode,
  9:string CampaignTypeDesc,
  10:string MfrNotes,
  11:HondaRecallCdValue MfrRecallStatus,
  12:string RecallDate,
  13:string RecallDescription,
  14:string RecallNumber,
  15:string RefreshDate,
  16:string RemedyDescription,
  17:string SafetyRiskDescription,
  18:string SortOrder,
}

struct HondaRecallCampaignType
{
  1:list<HondaRecallCampaign> Campaign
  2:string cd,
  3:string name,
  4:string num_campaigns,
  5:string sort_order,
}

struct HondaRecall
{
  1:HondaRecallCampaignType CampaignType,
  2:string division_cd,
  3:string  exterior_color_name,
  4:string interior_color_name,
  5:string model_group_name,
  6:string model_year,
  7:string trans,
  8:string trim,
  9:string VIN,
}

struct HondaRecallsResponse
{
  1:string error,
  2:HondaRecall Recall,
}

service HondaRecallService
{
  HondaRecallsResponse getRecallsbyVIN(1:HondaRecallsRequest hondaRecallsRequest),
}

struct PriceCode
{
  1:string sPriceCode,
  2:string sDescription,
  3:string sDisplayedDescription,
  4:string sPayType,
  5:double fRate,
  6:string sPayTypeLabel,
}

// DealerBuilt GM Savi

struct DealerBuiltSaviTroubleCode
{
  1:string serviceVisitId,
  2:string ecuId,
  3:string ecuName,
  4:string type,
  5:string dtcCodeId,
  6:string codeDescription,
  7:string statusCode,
  8:string statusDescription,
  9:string symptomCode,
  10:string symptomDescription,
  11:string modified,
  12:i32 id,
}

struct DealerBuiltSaviVehicleData
{
  1:string displayName,
  2:string value,
  3:string units,
  4:i32 id,
}

struct DealerBuiltSaviEcuCount
{
  1:string serviceVisitId,
  2:string controlModule,
  3:i32 dtcCount,
  4:string ecuId,
  5:string modified,
  6:i32 id,
}

struct DealerBuiltSaviServiceVisit
{
  1:string serviceVisitId,
  2:string vin,
  3:i32 odometer,
  4:string checkInDate,
  5:string checkInTime,
  6:string make,
  7:i32 modelYear,
  8:string model,
  9:string color,
  10:string engineLight,
  11:i32 dealerBac,
  12:string dealerName,
  13:string dongleType,
  14:string advisorName,
  15:string advisorGmin,
  16:string appointmentId,
  17:string repairOrderId,
  18:i32 dtcCount,
  19:list<DealerBuiltSaviTroubleCode> troubleCodes,
  20:list<DealerBuiltSaviVehicleData> vehicleData,
  21:list<DealerBuiltSaviEcuCount> ecuCounts,
  22:string modified,
  23:string modifiedBy,
  24:i32 id
  25:i32 repairOrderStatus,
}

struct DealerBuiltSaviResponse
{
  1:list<DealerBuiltSaviServiceVisit> results,
  2:string statusCode,
  3:map<string, list<string>> errors,
  4:string traceId,
}

service DealerBuiltSaviService
{
  DealerBuiltSaviResponse getServiceVisits(1:string dealerId, 2:string locationId, 3:string startDate, 4:string endDate, 5:string vin),
  DealerBuiltSaviResponse getServiceVisitsByVin(1:string dealerId, 2:string locationId, 3:string vin, 4:string serviceVisitId),
  DealerBuiltSaviResponse getServiceVisitByAppointment(1:string dealerId, 2:string locationId, 3:string appointmentId),
  DealerBuiltSaviResponse getServiceVisitByRepairOrder(1:string dealerId, 2:string locationId, 3:string roId),
  DealerBuiltSaviResponse setServiceVisitWithAppointment(1:string dealerId, 2:string locationId, 3:string serviceVisitId, 4:string appointmentId),
  DealerBuiltSaviResponse setServiceVisitWithRepairOrder(1:string dealerId, 2:string locationId, 3:string serviceVisitId, 4:string roId),
}

service PartsLaborService
{
  string getAuthorizationToken(1:i32 companyID),
  bool processResponse(1:i32 promiseID, 2:string jsonResponse)
}

service AudiAfterSalesService
{
  void processLeadForPromise(1:i32 iPromiseID)
}

enum MFAMethod
{
  Email = 1,
  SMS = 2,
  Skip = 3,  # temporarily putting this here until MFAAuth refactor
}

struct MFAContact
{
  1:MFAMethod preferred,
  2:string email,
  3:string phone_number,
  4:string verified_email,
  5:string verified_phone,
}

exception MFAException
{
  1:string message,
}

struct UserDevice
{
  1:string ip_address,
  2:string user_agent,
  3:string mac_address,
}

service MFAService
{
  // We use username here because the user is not logged in yet and therefore cannot access their login ID.
  bool sendToken(1:string username, 2:MFAMethod contact_method, 3:string contact_info, 4:bool force) throws (1:MFAException ex),
  // use login() with token
  bool verifyToken(1:string username, 2:MFAMethod contact_method, 3:string token),
  bool rememberMe(1:i32 login_id, 2:string device_fingerprint),
  bool disable(1:i32 login_id, 2:MFAMethod contact_method),
  bool enable(1:i32 login_id, 2:MFAMethod contact_method, 3:string contact_info) throws (1:MFAException ex),
  bool reset(1:i32 login_id),
  bool setPreferredMethod(1:i32 login_id, 2:MFAMethod contact_method) throws (1:MFAException ex),
  bool isEnabled(1:string username),
  MFAContact getContact(1:string username) throws (1:MFAException ex),
}


service HMADealerLocatorService
{
  string fetchDealerList()
}


struct RentalLocation {
  1:string name,
  2:string website,
  3:string address,
  4:string street,
  5:string city,
  6:string state,
  7:string country,
  8:string zip,
  9:string contact_details,
  10:string address_title,
  11:bool is_station,
  12:string station_id
}

struct RentalVehicleMPG {
  1:i32 city,
  2:i32 highway,
  3:i32 combined
}

struct RentalVehicle {
  1:string body_style_desc,
  2:string body_style_code,
  3:string model,
  4:string year,
  5:i32 available,
  6:RentalVehicleMPG mpg,
  7:string acriss_code,
  8:string vin,
  9:string classification,
  10:string stock_image,
  11:string make,
  12:string luggage_bags
}

struct RentalTimeslot {
  1:string start_datetime,
  2:string end_datetime
}

struct RentalVehicleAvailability {
  1:list<RentalTimeslot> available_slots,
  2:RentalVehicle vehicle
}

struct RentalVehicleAvailabilityByBrand {
  1:string brand,
  2:list<RentalVehicleAvailability> available_models
}

struct RentalOperationHours {
  1:string availabilty_start,
  2:string availability_end
}

struct RentalOperationDay {
  1:bool is_open,
  2:bool is_today,
  3:string day_of_week,
  4:string calendar_date,
  5:list<RentalOperationHours> hours_of_operation
}

struct RentalOperation {
  1:list<RentalOperationDay> standard_hours,
  2:list<RentalOperationDay> exception_hours
}

struct RentalStationAvailability {
  1:RentalLocation station,
  2:RentalOperation operational_hours,
  3:list<RentalVehicleAvailabilityByBrand> vehicles
}

struct RentalTaxDetail {
  1:double amount,
  2:double rate,
  3:string name,
  4:string type
}

struct RentalTax {
  1:double total_tax,
  2:list<RentalTaxDetail> details
}

struct RentalFeeDetail {
  1:double amount,
  2:double rate,
  3:string type,
  4:string frequency
}

struct RentalFee {
  1:double total_fee,
  2:list<RentalFeeDetail> details
}

struct RentalPriceBreakup {
  1:double total_rate,
  2:RentalTax tax,
  3:RentalFee fee
}

struct RentalRate {
  1:double rate_amount,
  2:string unit,
  3:i32 quantity,
  4:double rate,
  5:RentalTax tax
}

struct RentalVehiclePrice {
  1:i32 deposit_amount,
  2:i32 duration,
  3:RentalLocation station,
  4:RentalVehicle vehicle,
  5:string rental_price_id,
  6:double estimated_total,
  7:RentalPriceBreakup price_breakup,
  8:RentalTimeslot booking_timeslot,
  9:list<RentalRate> rates
}

struct RentalAvailability {
  1:RentalLocation dealer,
  2:list<RentalStationAvailability> stations
}

struct RentalPrices {
  1:RentalLocation dealer,
  2:list<RentalVehiclePrice> vehicles
}

struct RentalReservation {
  1:string start_datetime,
  2:string end_datetime,
  3:RentalVehiclePrice vehicle_price,
  4:string reservation_id
}

service RentalService
{
  list<string> getRentalServiceOptions(),
  list<string> findReservations(1:i32 promise_id),
  RentalReservation getReservation(1:i32 company_id, 2:string rental_provider, 3:i32 promise_id)
  bool cancelReservation(1:i32 company_id, 2:string rental_provider, 3:i32 promise_id),
  bool saveReservation(1:i32 company_id, 2:string rental_provider, 3:i32 promise_id,
                       4:string start_datetime, 5:string end_datetime, 6:RentalVehiclePrice vehicle_price)
  bool saveReservationUpdates(1:i32 company_id, 2:string rental_provider, 3:i32 promise_id)
  list<string> getBodyStyles(1:string rental_provider),
  list<string> getClassifications(1:string rental_provider),
  RentalAvailability getVehicleAvailability(1:i32 company_id, 2:string rental_provider, 3:optional i32 duration,
                                            4:optional string start_date, 5:optional string end_date,
                                            6:optional list<string> body_style,
                                            7:optional list<string> classification),
  RentalPrices getVehiclePricing(1:i32 company_id, 2:string rental_provider, 3:optional i32 duration,
                                 4:optional string start_date, 5:optional string end_date,
                                 6:optional list<string> body_style, 7:optional list<string> model_name,
                                 8:optional i32 deposit)
}