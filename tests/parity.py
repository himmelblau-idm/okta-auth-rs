#!/usr/bin/env python3
"""Compare compiled Rust with actual pinned TypeScript, never fixed Rust expectations."""
import json, pathlib, subprocess, base64
ROOT = pathlib.Path(__file__).resolve().parents[1]
cases = []
def case(fn, *args): cases.append({'function': fn, 'args': args})
for value in [None, False, True, 0, 1, '', 'code', [], {}, {'nested': 'value'}]:
    for mutable in [None, False, True]:
        field = {'name':'answer','value':value}
        if mutable is not None: field['mutable']=mutable
        case('divideActionParamsByMutability', {'name':'identify','value':[field]})
case('divideActionParamsByMutability', {'name':'external','href':'https://example.invalid'})
case('divideActionParamsByMutability', [{'name':'a','value':[]},{'name':'b','value':[{'name':'stateHandle','mutable':False,'value':'s'}]}])
case('unwrapFormValue', {'name':'credentials','value':{'form':{'value':[{'name':'passcode','required':True}]}}})
case('unwrapFormValue', {'name':'factor','options':[{'label':'TOTP','value':{'form':{'value':[{'name':'id','value':'a'},{'name':'flag','value':False}]}}}]})
case('unwrapFormValue', {'name':'x','value':None,'required':False,'extra':{'nested':{'x':0}}})
case('unwrapFormValue', ['a', False, 0, {'name':'x','value':''}])
# Inputs exercise translated WebAuthn conversions and optional chaining.
for enrollments in [[], [{'key':'webauthn','credentialId':'AAH_','profile':{'transports':['usb']}}], [{'key':'webauthn','credentialId':'AQID','transports':['internal']}], [{'key':'password'}]]:
    case('buildCredentialRequestOptions', {'challenge':'AP8B','rpId':'example.invalid','userVerification':'required','hints':['security-key']}, enrollments)
    case('buildCredentialCreationOptions', {'rp':{'id':'example.invalid','name':'Fixture'},'user':{'id':'AQID','name':'user','displayName':'User'},'challenge':'AP8B','pubKeyCredParams':[{'type':'public-key','alg':-7}],'attestation':'none'}, enrollments)
case('getAssertion', {'id':'credential','response':{'clientDataJSON':[0,1,255],'authenticatorData':[1,2,3],'signature':[255,254,253]}})
case('getAttestation', {'id':'credential','response':{'clientDataJSON':[1,2,3],'attestationObject':[255,254,253]}})
for text in ['', 'abc', 'MFA 🔐', '你好']:
    case('getOidcHash',text)
    case('base64UrlToString',base64.urlsafe_b64encode(text.encode()).decode().rstrip('='))
for path in ['$.user.name','$.items[1].id','$.missing','', '$.user.name[0]']:
    case('jsonpath',{'path':path,'json':{'user':{'name':'Ada'},'items':[{'id':1},{'id':2}]}})
case('toQueryString',{'username':'a+b@example.invalid','scope':'openid email','code':'🔥 & =','empty':''})
case('getPostData',{'options':{}},{'clientId':'client','redirectUri':'http://localhost/callback','interactionCode':'ic','codeVerifier':'verifier'})
case('getPostData',{'options':{}},{'clientId':'client','redirectUri':'http://localhost/callback','authorizationCode':'code','codeVerifier':'verifier'})
claims={'iss':'https://example.invalid','aud':'client','nonce':'nonce','iat':1,'exp':4102444800}
validation={'clientId':'client','issuer':'https://example.invalid','nonce':'nonce'}
sdk={'options':{'maxClockSkew':0}}
case('validateClaims',sdk,claims,validation)
case('validateClaims',sdk,{**claims,'aud':['other','client']},validation)
signature=json.loads((ROOT/'tests/fixtures/signature.json').read_text())
signature_positive_index = len(cases)
case('verifySignature',signature['token'],signature['key'])
parts=signature['token'].split('.')
parts[1]=base64.urlsafe_b64encode(b'{"sub":"tampered"}').decode().rstrip('=')
case('verifySignature','.'.join(parts),signature['key'])
# Error normalization retains the pinned SDK object shape; native diagnostics
# are carried separately and must not change these enumerable properties.
for status in [400, 404, 429, 503]:
    for body in [None, {'error': 'invalid_grant'}, {'error': 'invalid_grant', 'error_description': 'fixture'}, {'errorCode': 'E0000004'}, {'errorCode': 'invalid code', 'errorSummary': 'fixture'}]:
        case('formatHttpError', {'options': {}}, {'status': status, 'headers': {}, 'responseText': '<html>fixture</html>' if body is None else json.dumps(body)})
positive_count=len(cases)
for changed in [{'iss':'wrong'},{'aud':'wrong'},{'nonce':'wrong'},{'exp':0},{'iat':4102444801}]:
    case('validateClaims',sdk,{**claims,**changed},validation)
case('base64UrlToString','!invalid!')
payload = ''.join(json.dumps(c)+'\n' for c in cases)
def run(cmd):
    p = subprocess.run(cmd,input=payload,text=True,capture_output=True,cwd=ROOT,check=True)
    return [json.loads(x) for x in p.stdout.splitlines()]
js = run(['node','tests/oracle/run.cjs'])
rs = run(['target/debug/examples/parity'])
assert len(js)==len(rs)==len(cases)
for i,(expected,actual) in enumerate(zip(js,rs)):
    assert expected==actual, (cases[i],expected,actual)
assert all('ok' in result for result in js[:positive_count]), 'positive oracle fixture failed'
assert all('error' in result for result in js[positive_count:]), 'negative oracle fixture unexpectedly succeeded'
assert rs[signature_positive_index] == {'ok':True}, 'valid signature must verify'
assert rs[signature_positive_index+1] == {'ok':False}, 'tampered payload must fail verification'
print(f'{len(cases)} upstream/Rust comparisons passed')
