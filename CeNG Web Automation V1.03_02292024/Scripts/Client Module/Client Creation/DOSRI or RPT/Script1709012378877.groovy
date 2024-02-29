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

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_DOSRI_dosri'))

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_NONE_dosri'))

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_YES_dosri'))

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_NONE_dosri'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--BrotherBrother-in-LawCousi_e1fca2'), 
    '765', true)

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_Nameof Staff_txtNameofStaff'), 
    'Trafalgar Law')

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--Account AnalystAccounting _ce5c08'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--Account AnalystAccounting _ce5c08'), 
    '3675', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--CARD Bank, Inc.CARD SME Ba_7bc8fb'), 
    '2693', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_-- Select --CARD MRI MemberCARD MRI _8bd785'), 
    '1807', true)

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_Referredby_txtReferredbyOtherPrimary'), 
    'Bepo the Bear')

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/i_Referredby_relationshipIcon'))

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_None_drone2'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--BrotherBrother-in-LawCousi_e1fca2_1'), 
    '763', true)

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_Nameof Staff_txtNameofStaff2'), 
    'Eustass Kidd')

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--Account AnalystAccounting _ce5c08_1'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--Account AnalystAccounting _ce5c08_1'), 
    '3669', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--CARD Bank, Inc.CARD SME Ba_7bc8fb_1'), 
    '2696', true)

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_CID_txtPrimaryImmediateFamCID'), 
    '10000123')

WebUI.sendKeys(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_CID_txtPrimaryImmediateFamCID'), 
    Keys.chord(Keys.ENTER))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--BrotherDaughterFatherMothe_ec9985'), 
    '3660', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/i_Center_btnrow1'))

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_Center_txtImmediateFamCID2'), 
    '10000150')

WebUI.sendKeys(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_Center_txtImmediateFamCID2'), 
    Keys.chord(Keys.ENTER))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--BrotherDaughterFatherMothe_ec9985_1'), 
    '3664', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/i_Center_btnrow1'))

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_Center_txtImmediateFamCID3'), 
    '10000145')

WebUI.sendKeys(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/input_Center_txtImmediateFamCID3'), 
    Keys.chord(Keys.ENTER))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/select_--Select--BrotherDaughterFatherMothe_ec9985_1_2'), 
    '3659', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/DOSRI or RPT/Page_Core/button_NEXT'))

