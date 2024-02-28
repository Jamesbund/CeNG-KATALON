import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import java.text.SimpleDateFormat as SimpleDateFormat
import java.util.Calendar as Calendar
import java.util.Date as Date

///////////////////////////
WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_with Disability_withdisability'))

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_YES_withdisability'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--NewTransferred'), 
    '1820', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_-- Select --4Ps BeneficiaryArtistChi_05c006'), 
    '1787', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_-- Select --Atty.Dr.Eng.Hon.MrMrsMs'), 
    '155', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--BisexualGayLesbianQueerTra_c208c0'), 
    '76', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--BisexualGayLesbianQueerTra_c208c0'), 
    '0', true)

//Marital Status MARRIED
WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/select_--Select--AnnulledLive-inMarriedSepa_54a068'), 
    '162', true)

//Marital Status SINGLE
///WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/select_--Select--AnnulledLive-inMarriedSepa_54a068'),
//'165', true)
WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_FirstName_txtmothersMaidenFName'), 
    'Hayley')

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_MiddleName_txtmothersMaidenMName'), 
    'N')

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_LastName_txtmothersMaidenLName'), 
    '')

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/div_Please ensure that the middle name cont_98c59e'))

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_MiddleName_txtmothersMaidenMName'), 
    'Nicole')

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_LastName_txtmothersMaidenLName'), 
    'Williams')

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--AglipayanBorn AgainEvangel_87720a'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--AglipayanBorn AgainEvangel_87720a'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--AglipayanBorn AgainEvangel_87720a'), 
    '422', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--AglipayanBorn AgainEvangel_87720a'), 
    '422', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--No Formal EducationElement_aa261c'), 
    '197', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--No Formal EducationElement_aa261c'), 
    '197', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--BusinessGovernment Employe_69ab64'), 
    '441', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--BusinessGovernment Employe_69ab64'), 
    '441', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--Accommodation and Food Ser_e45bf9'), 
    '699', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--Accommodation and Food Ser_e45bf9'), 
    '699', true)

///// Birthday
// Get the current date
Date currentDate = new Date()

// Create a Calendar instance and set it to the current date
Calendar calendar = Calendar.getInstance()

calendar.setTime(currentDate)

// Subtract 20 years from the current date
calendar.add(Calendar.YEAR, -20)

// Get the resulting date
Date dateTwentyYearsBack = calendar.getTime()

// Format the date as needed
SimpleDateFormat dateFormat = new SimpleDateFormat('MM/dd/yyyy' // Adjust the format as per your requirements
)

String formattedDate = dateFormat.format(dateTwentyYearsBack)

// Input the formatted date into the calendar field
WebUI.setText(findTestObject('Captured Object/Personal Information/Page_Core/Page_Core/input_Birthday_txtbday'), formattedDate)

WebUI.sendKeys(findTestObject('Captured Object/Personal Information/Page_Core/Page_Core/input_Birthday_txtbday'), Keys.chord(Keys.ENTER))

///////////
///////////
WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--      Select      --Abra - CAR (Co_3be376'), 
    '10488', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--      Select      --Abra - CAR (Co_3be376'), 
    '10488', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--      Select      --AlaminosBayBia_8f7d3f'), 
    '10571', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--      Select      --AlaminosBayBia_8f7d3f'), 
    '10571', true)

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_0.00'), '09091231231')

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_TelephoneNumber_txtTelephoneNumber'), 
    '(12) 312-3123')

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_TelephoneNumber_txtTelephoneNumber'))

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_EmailAddress_txtEmailAddress'), 
    'anitamaxwynn@sample.com')

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_TIN_txtTinID'), '123-123-123-12321')

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_TIN_txtTinID'))

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_SSSNo_txtSSSNo'), '12-3123123-1')

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_SSSNo_txtSSSNo'))

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_PhilhealthNo_txtPhilhealth'), 
    '12-3123123-123')

WebUI.setText(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_Pag-IBIGNo_txtPagIbig'), 
    '1231-2312-3123')

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--PAETE (REGULAR)Sampalok 1P_2adbb5'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--PAETE (REGULAR)Sampalok 1P_2adbb5'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--PAETE (REGULAR)Sampalok 1P_2adbb5'), 
    '97', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--Select--PAETE (REGULAR)Sampalok 1P_2adbb5'), 
    '97', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--      Select      --BAGONG POOK (S_ac7106'), 
    '708', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/select_--      Select      --BAGONG POOK (S_ac7106'), 
    '708', true)

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_YES_CARDScholar'))

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/input_YES_FormerEmployee'))

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/button_NEXT'))

