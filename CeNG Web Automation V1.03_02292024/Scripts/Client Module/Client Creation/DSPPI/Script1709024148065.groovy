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



WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - ARMM8 - Il_f2aada'), 
    '8', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - 7 o higit _f836c6'), 
    '9', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Ika-6 na b_ee969e'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Magaang ma_b84cbb'), 
    '5', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Magaang ma_840447'), 
    '7', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Wala7 - Oo'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Iba pa5 - _f3691f'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Hindi13 - Oo'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Hindi11 - Oo'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Hindi8 - Oo'), 
    '0', true)

WebUI.delay(5)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Wala7 - Oo'), 
    '7', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Iba pa5 - _f3691f'), 
    '7', true)

WebUI.click(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/td_--      Select      --0 - Hindi13 - Oo'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Hindi13 - Oo'), 
    '13', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Hindi11 - Oo'), 
    '11', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--      Select      --0 - Hindi8 - Oo'), 
    '8', true)

WebUI.delay(5)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--Select--GLIP 1GLIP 2IPPaglambo'), 
    '674', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/select_--Select--Adasen InlaudAetaAetaAlang_82eebf'), 
    '4511', true)

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/Loan Creation/DSPPI/Page_Core/button_NEXT'))

