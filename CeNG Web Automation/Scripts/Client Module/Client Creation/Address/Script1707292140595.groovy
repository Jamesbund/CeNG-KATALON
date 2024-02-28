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

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/select_--      Select      --Abra - CAR (Co_3be376'), 
    '10488', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/select_--      Select      --AlaminosBayBia_8f7d3f'), 
    '10571', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/select_--      Select      --AlaminosBayBia_8f7d3f'), 
    '10571', true)

WebUI.verifyElementText(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/input_PostalCode_txtCusAddZipCode'), 
    '')

WebUI.selectOptionByValue(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/select_AtisanBagong Bayan Ii-A (Pob.)Bagong_361eda'), 
    '13059', true)

WebUI.setText(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/input_HouseNo.BldgStreetSubdivision_txthous_59daa2'), 
    '777. Purok 12, Limawasa Street')

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/input_Same with the Present AddressPhysical_c9b861'))

WebUI.delay(8)

WebUI.click(findTestObject('Object Repository/Loan Creation/Client Creation/Address/Page_Core/button_NEXT'))

