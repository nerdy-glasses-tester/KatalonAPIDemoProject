import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

response1 = WS.sendRequest(findTestObject('APIChainRestServiceWithBaseURIVariable/ListUsers', [('baseuri') : GlobalVariable.baseuri]))

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(response1.getResponseBodyContent())

def value = result.data[1].first_name

println('Value extracted is ' + value)

GlobalVariable.username = value

println('Global variable username is ' + GlobalVariable.username)

WS.sendRequestAndVerify(findTestObject('APIChainRestServiceWithBaseURIVariable/UpdateUser', [('username') : GlobalVariable.username
            , ('baseuri') : GlobalVariable.baseuri]))

