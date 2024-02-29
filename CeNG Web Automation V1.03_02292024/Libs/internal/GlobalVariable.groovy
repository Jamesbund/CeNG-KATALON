package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object UsernameTeller
     
    /**
     * <p></p>
     */
    public static Object PasswordTeller
     
    /**
     * <p></p>
     */
    public static Object ClientFirstName
     
    /**
     * <p></p>
     */
    public static Object ClientMiddleName
     
    /**
     * <p></p>
     */
    public static Object ClientLastName
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += TestCaseMain.getParsedValues(RunConfiguration.getOverridingParameters(), selectedVariables)
    
            UsernameTeller = selectedVariables['UsernameTeller']
            PasswordTeller = selectedVariables['PasswordTeller']
            ClientFirstName = selectedVariables['ClientFirstName']
            ClientMiddleName = selectedVariables['ClientMiddleName']
            ClientLastName = selectedVariables['ClientLastName']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
