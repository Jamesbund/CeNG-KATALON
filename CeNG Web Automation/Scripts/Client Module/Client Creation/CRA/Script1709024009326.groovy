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

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/select_Select      51015'), 
    '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/select_Select      51015_1'), 
    '5', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/select_Select      51020'), 
    '8', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/td_Select      0030'))

WebUI.scrollToElement(findTestObject('Loan Creation/Client Creation/CRA/Page_Core/td_Select      0030'), 0)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/select_Select      01020'), 
    '11', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/td_Select      51015'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/select_Select      0030'), 
    '14', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/td_Select      51015'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/select_Select      51015_1_2'), 
    '17', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/CRA/Page_Core/select_Select      51015_1_2_3'), 
    '20', true)

WebUI.delay(8)

WebUI.click(findTestObject('Object Repository/Loan Creation/KYC/Page_Core/button_NEXT'))

