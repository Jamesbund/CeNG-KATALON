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

WebUI.click(findTestObject('Captured Object/Other Information/Page_Core/input_CARD Member_drone3'))

WebUI.click(findTestObject('Captured Object/Other Information/Page_Core/input_YES_drone3'))

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_FirstName_txtSpousefname'), 
    'Natasha')

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_MiddleName_txtSpousemname'))

WebUI.doubleClick(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_MiddleName_txtSpousemname'))

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_MiddleName_txtSpousemname'), 
    'The')

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_LastName_txtSpouselname'), 
    'Romanoff')

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--FemaleMale'), 
    '74', true)

///// DATE OF MARRIAGE
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
SimpleDateFormat dateFormat = new SimpleDateFormat('MM/dd/yyyy')

String formattedDate = dateFormat.format(dateTwentyYearsBack)

// Input the formatted date into the calendar field
WebUI.setText(findTestObject('Loan Creation/Client Creation/Other Information/Page_Core/input_Dateof Marriage_txtSpouseDateOfMarraige'), 
    formattedDate)

WebUI.sendKeys(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_Dateof Marriage_txtSpouseDateOfMarraige'), 
    Keys.chord(Keys.ENTER))

// Input the formatted date into the calendar field
WebUI.setText(findTestObject('Loan Creation/Client Creation/Other Information/Page_Core/input_Birthday_txtSpousebday'), 
    formattedDate)

WebUI.sendKeys(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_Birthday_txtSpousebday'), 
    Keys.chord(Keys.ENTER))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--Aborlan - PalawanAbra De I_52077c'), 
    '3560', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--BusinessGovernment Employe_69ab64'), 
    '491', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--EmploymentSalariesIncome f_b99596'), 
    '1759', true)

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_SourceOf Income_txtCompanyname'), 
    'CMIT')

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--No Formal EducationElement_aa261c'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--No Formal EducationElement_aa261c'), 
    '197', true)

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_CellphoneNumber_txtSpouseCpNmber'), 
    '09091231231')

WebUI.sendKeys(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_CellphoneNumber_txtSpouseCpNmber'), 
    Keys.chord(Keys.ENTER))

////ROW 1 Beneficiary

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_Remarks_txtBeneficiaryName1'), 
    'Bruce Banner')

WebUI.setText(findTestObject('Captured Object/Other Information/Page_Core/input_Remarks_txtBeneficiaryBday1'), formattedDate)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--BrotherBrother-in-LawCousi_e1fca2'), 
    '763', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--FemaleMale_1'), 
    '75', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--No Formal EducationElement_aa261c_1'), 
    '203', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/div_withDisability If Yes, specify the kind_30fc26'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--YESNO'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--YESNO_1'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--BeneficiaryLegal Dependent_a71cea'), 
    '1803', true)

//////ROW 2 Beneficiary

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/i_Remarks_beneRow1'))

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_Remarks_txtBeneficiaryName2'), 
    'Peter Parker')

WebUI.setText(findTestObject('Captured Object/Other Information/Page_Core/input_Remarks_txtBeneficiaryBday2'), formattedDate)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--BrotherBrother-in-LawCousi_e1fca2_1'), 
    '758', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--FemaleMale_1_2'), 
    '75', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--No Formal EducationElement_aa261c_1_2'), 
    '203', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/div_Additional Beneficiary If any'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--No Formal EducationElement_aa261c_1_2'), 
    '180', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--NAPrimarySecondary'), 
    '1800', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--YESNO_1_2'), 
    '1', true)

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_Remarks_txtDisability2'), 
    'ADHD')

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--YESNO_1_2_3'), 
    '1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--BeneficiaryLegal Dependent_a71cea_1'), 
    '1803', true)


///////////////ROW 3 Beneficiary

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/i_Remarks_beneRow1'))

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_Remarks_txtBeneficiaryName3'))

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/input_Remarks_txtBeneficiaryName3'), 
    'Wanda Vision')

WebUI.setText(findTestObject('Captured Object/Other Information/Page_Core/input_Remarks_txtBeneficiaryBday3'), formattedDate)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/label_Relationship'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--BrotherBrother-in-LawCousi_e1fca2_1_2'), 
    '766', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--FemaleMale_1_2_3'), 
    '75', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--No Formal EducationElement_aa261c_1_2_3'), 
    '199', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--NAPrimarySecondary_1'), 
    '1801', true)

//WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/div_PersonalInformationAddressID PresentedD_487406'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--YESNO_1_2_3_4'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--YESNO_1_2_3_4_5'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/select_--Select--BeneficiaryLegal Dependent_a71cea_1_2'), 
    '1804', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Other Information/Page_Core/button_NEXT'))

