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
import java.util.Date as Date
import java.util.Calendar as Calendar

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.9.2.15:8880/eSystemNextGenWebApp/pages/Login')

WebUI.maximizeWindow()

String UsernameTeller = GlobalVariable.UsernameTeller

WebUI.setText(findTestObject('Object Repository/Loan Creation/Page_Card eSystemNextGen/input_Username_txtUsername'), UsernameTeller)

WebUI.setEncryptedText(findTestObject('Object Repository/Loan Creation/Page_Card eSystemNextGen/input_Password_txtPassword'), 
    '+occZAnJluM0nv9I0v7U3uaH2ilQtLRo')

WebUI.sendKeys(findTestObject('Object Repository/Loan Creation/Page_Card eSystemNextGen/input_Password_txtPassword'), Keys.chord(
        Keys.ENTER))

WebUI.verifyElementVisible(findTestObject('Page_Core/h4_Dashboard'))

WebUI.mouseOver(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/span_Client'))

WebUI.verifyElementVisible(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/span_Client'))

WebUI.delay(10)

WebUI.click(findTestObject('Captured Object/Page_Core/span_Client'))

WebUI.delay(3)

WebUI.click(findTestObject('Loan Creation/Client Creation/Page_Core/a_ClientCreation'))

WebUI.click(findTestObject('Loan Creation/Client Creation/Page_Core/a_ClientCreation'))

WebUI.click(findTestObject('Loan Creation/Client Creation/Page_Core/a_ClientCreation'))

WebUI.acceptAlert()

WebUI.setText(findTestObject('Loan Creation/Page_Core/input_FirstName_txtFname'), 'First Name 1')

WebUI.setText(findTestObject('Loan Creation/Page_Core/input_LastName_txtLname'), 'Last Name 2')

WebUI.setText(findTestObject('Loan Creation/Page_Core/input_MiddleName_txtMname'), 'Middle Name 1')

WebUI.verifyElementText(findTestObject('Loan Creation/Client Creation/Page_Core/input_Birthday_txtdoBirth'), '')

////////////////// Getting Current System Date
// Get the current system date
Date currentDate = new Date()

// Format the date as needed
SimpleDateFormat dateFormat = new SimpleDateFormat('MM-dd-yyyy')

String formattedDate = dateFormat.format(currentDate)

// Print or use the formatted date
println('Current System Date: ' + formattedDate)

WebUI.setText(findTestObject('Captured Object/Personal Information/Page_Core/input_Birthday_txtdoBirth'), formattedDate)

/////////////////////
WebUI.click(findTestObject('Captured Object/Personal Information/Page_Core/button_SEARCH'))

WebUI.clearText(findTestObject('Loan Creation/Page_Core/input_FirstName_txtFname'))

WebUI.delay(5)

WebUI.click(findTestObject('Captured Object/Personal Information/Page_Core/button_OK'))

WebUI.clearText(findTestObject('Loan Creation/Page_Core/input_MiddleName_txtMname'))

WebUI.clearText(findTestObject('Loan Creation/Page_Core/input_LastName_txtLname'))

WebUI.delay(5)

//retrieve the clients full name from global variable
String firstName = GlobalVariable.ClientFirstName

String middleName = GlobalVariable.ClientMiddleName

String lastName = GlobalVariable.ClientLastName

WebUI.setText(findTestObject('Loan Creation/Page_Core/input_FirstName_txtFname'), firstName)

WebUI.setText(findTestObject('Loan Creation/Page_Core/input_MiddleName_txtMname'), middleName)

WebUI.setText(findTestObject('Loan Creation/Page_Core/input_LastName_txtLname'), lastName)

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/button_SEARCH'))

WebUI.click(findTestObject('Object Repository/Captured Object/Personal Information/Page_Core/button_CREATE'))

WebUI.delay(10)

WebUI.verifyElementPresent(findTestObject('Captured Object/Personal Information/h4_Personal Information'), 0)

WebUI.scrollToElement(findTestObject('Captured Object/Personal Information/h4_Personal Information'), 0)

WebUI.delay(15)

