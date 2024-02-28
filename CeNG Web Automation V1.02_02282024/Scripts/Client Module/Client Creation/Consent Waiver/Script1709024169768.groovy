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

import com.kms.katalon.core.testobject.TestObject

import org.openqa.selenium.By

import org.openqa.selenium.JavascriptExecutor;

import com.kms.katalon.core.webui.driver.DriverFactory

import org.openqa.selenium.chrome.ChromeDriver;

import org.openqa.selenium.WebElement



def driver = DriverFactory.getWebDriver()


WebElement health_dec = driver.findElement(By.id("healthDeclarationYes"))
((JavascriptExecutor) driver).executeScript("arguments[0].click();", health_dec);


WebElement Sumasangayonako1 = driver.findElement(By.id("txtiAgree"))
((JavascriptExecutor) driver).executeScript("arguments[0].click();", Sumasangayonako1);


WebUI.mouseOver(findTestObject('Object Repository/Loan Creation/Consent Waiver/Page_Core/div_PAGSANG-AYON ATPAGTALIKOD (CONSENT AND _48483f'))

WebUI.mouseOver(findTestObject('Object Repository/Loan Creation/Consent Waiver/Page_Core/p_Ako si   aynagpapatunay na ang lahat ng i_cea5bf'))


WebUI.click(findTestObject('Object Repository/Loan Creation/Consent Waiver/Page_Core/div_Sumasang-ayonako'))

WebElement Sumasangayonako2 = driver.findElement(By.id("txtiAgreeCBU"))
((JavascriptExecutor) driver).executeScript("arguments[0].click();", Sumasangayonako2);

WebUI.mouseOver(findTestObject('Object Repository/Loan Creation/Consent Waiver/Page_Core/h4_CAPITAL BUILD-UP(CBU)MICRO-SAVINGS (ACCO_4822a1'))

/*
WebUI.click(findTestObject('Object Repository/Loan Creation/Consent Waiver/Page_Core/button_NEXT'))


WebUI.click(findTestObject('Object Repository/Loan Creation/Consent Waiver/Page_Core/button_OK'))



*/
