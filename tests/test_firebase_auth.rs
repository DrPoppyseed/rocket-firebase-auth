mod common;

use crate::common::{mock_jwk_issuer, setup_mock_server, TEST_JWKS_URL};
use once_cell::sync::Lazy;
use rocket_firebase_auth::{
    errors::{Error, InvalidJwt},
    jwk::Jwk,
    FirebaseAuth,
};

static SCENARIO_MISSING_KID: Lazy<Scenario> = Lazy::new(|| {
    Scenario {
    token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwiaWF0IjoxNTE2MjM5MDIyfQ.L8i6g3PfcHlioHCCPURC9pmXT7gdJpx3kOoyAfNUwCc".to_string(),
    jwk_n: "qPSOdIwB1PidjFSY_dLckUu1Y4rPnP5nvOtMy_wMToekETe-h3P_FGVh_OA8E1J6stkeoWpqigfzuMIA6Ccc5BGC-xrt3jYgWSGtES8dKrPLjSFRmKOGk6IuosGvILpNRT-a9gocZOnRDGU9sE04vM2OfLQVc9kk-7tCWIirbu58DkpTYySZMaq-qLrbNUZg8w9MGZ0_wQabhiGaM1wfFJ9lV6uqL4CPkcdMvhaTQMV7lZStbApYUFiPBzIeKW6jrfB-dlkk08g_eluPkQHlHEGtH_5pTnBm9RIwzhHhP4hddc6LzdWAVfDe2DPrrFDfmUv45ejkIf4wmAreodjfPw".to_string(),
    kid: "test_private_kid".to_string(),
    ..Default::default()
}
});

static SCENARIO_HAPPY_PATH: Lazy<Scenario> = Lazy::new(|| {
    Scenario {
        token: "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Rfa2lkIn0.eyJzdWIiOiJzb21lLXVpZCIsImF1ZCI6ImR1bW15LXByb2plY3QtaWQiLCJpYXQiOjE2NjcwOTc2MjMsImV4cCI6NTAwNzA5OTYyMywiaXNzIjoiaHR0cHM6Ly9zZWN1cmV0b2tlbi5nb29nbGUuY29tL2R1bW15LXByb2plY3QtaWQifQ.A0ljz9xFb9p2CC6MzOlm-In-1wLIbbZVx3YgSTeajxEIYdrqNBVp-wh00mfwPM9ZGtuPmOUP8wbEzY_XjClNFCWdExmRY-UTLga0IeAekDgo8uWgr4g3cVQ5-RSK-mSDl1svzaZBq2YYyWpYZkfcsp2eoJhS04A4t9d_ADnvzNTC9YlPAyDp6DeDvyUE9K6B6_xIj2murScOz9CKijJq1rzItl1Ip8JVlNrEVFeiF5a7MZTMZYkV6ZN31hOCqc1N4eIhxHJ3ZvvM6YbZva-pdFsu_XGroYIb4Ar_hDpa3i4KrM517mp9e_nocGFKkjfQpNUnp0lsJ_PzRZZvHZ5rrQ".to_string(),
        jwk_n: "XiqTPp559nQUm1dxEecu1HT_yD0Vv4OY9uVG-YNCy8NU2ceaqoUfF05luJ9xtmf-D3PQjpwSouA2ejxr_GSD2VQ99our7CDe5t5jdNEXyXSrXJDua5-lEETT-fOmnAuhti9m86w7Jd2HafJCq9sGEuQc4-CM1-2faerEGGl9iyNGQEZBnS_ExDJOJnbI8J6JZnWhL_rjwkVcM6_MX9bDA1Hi4fosMW1V-WhnZUJ5-WvNfX_2feXfQ0qq5kH9BWhbYCxwwT2d_0xkdL0aItEB_nqZSS9yIDiGQATjf_P708dEo9A0tVZg9HUtLY5k4-JCrOUayJOYJ5b-YGETzSbELw".to_string(),
        kid: "test_kid".to_string(),
        public_key: Some(r#"-----BEGIN RSA PUBLIC KEY-----
MIIBITANBgkqhkiG9w0BAQEFAAOCAQ4AMIIBCQKCAQBeKpM+nnn2dBSbV3ER5y7UdP/IPRW/g5j25Ub5g0LLw1TZx5qqhR8XTmW4n3G2Z/4Pc9COnBKi4DZ6PGv8ZIPZVD32i6vsIN7m3mN00RfJdKtckO5rn6UQRNP586acC6G2L2bzrDsl3Ydp8kKr2wYS5Bzj4IzX7Z9p6sQYaX2LI0ZARkGdL8TEMk4mdsjwnolmdaEv+uPCRVwzr8xf1sMDUeLh+iwxbVX5aGdlQnn5a819f/Z95d9DSqrmQf0FaFtgLHDBPZ3/TGR0vRoi0QH+eplJL3IgOIZABON/8/vTx0Sj0DS1VmD0dS0tjmTj4kKs5RrIk5gnlv5gYRPNJsQvAgMBAAE=
-----END RSA PUBLIC KEY-----"#.to_string()),
        private_key: Some(r#"-----BEGIN RSA PRIVATE KEY-----
MIIEoQIBAAKCAQBeKpM+nnn2dBSbV3ER5y7UdP/IPRW/g5j25Ub5g0LLw1TZx5qq
hR8XTmW4n3G2Z/4Pc9COnBKi4DZ6PGv8ZIPZVD32i6vsIN7m3mN00RfJdKtckO5r
n6UQRNP586acC6G2L2bzrDsl3Ydp8kKr2wYS5Bzj4IzX7Z9p6sQYaX2LI0ZARkGd
L8TEMk4mdsjwnolmdaEv+uPCRVwzr8xf1sMDUeLh+iwxbVX5aGdlQnn5a819f/Z9
5d9DSqrmQf0FaFtgLHDBPZ3/TGR0vRoi0QH+eplJL3IgOIZABON/8/vTx0Sj0DS1
VmD0dS0tjmTj4kKs5RrIk5gnlv5gYRPNJsQvAgMBAAECggEAXPsdOY+yTjCAyIKn
G05zZ0W/6zCl8N04hVIPqwB5TEor1n7JseaQtKqstoh59+rnasqo/KgPntRV9o0C
880sg8QzCucPc7FhaAXfntF382xIaLaTNaIFkvLjfMOhmCPEIejcd29xWApOU8br
Hla+wJiODlUDvZLc/fDagGBpnqCZ9YrJnnGN3Z+arJ4NBD3FeiTJdjC/phJEioWZ
gPpA7Lzkst18oIlWIPMW44B1Ng4oaBv2cyIdcQSvtsOrZoeGwdmIOz2lFI73GPfA
d5orQeTEPMAQJ31vj8yTyTtT0sq8wEX6RpW7/Q3paHPcDlv1FXpL1G1FXu62vqza
6px+AQKBgQCrazVmMYDK7mGogeKPOxaF9SIPR0n4GK3/XonUPUdxsPcrUWHKDG8K
LhTMKV+o86eHZjoFrbpCIvgT7hBzvuL7VyI84neZPLkmAOVG0l2gk3++BvLGXWb4
/Ft6FT1BGhp3OLy7YpM5aPof0PKCfeFLjzFHRwTtAAiBBWg27W6SIQKBgQCMoTUx
NpjukmIhn2rjBDJuQTZwik7qXpwZd4Z6wGefp37yv1Sgj+Bcx7bCi9TLLG1X0hLV
wf1n0ZsXmsmAP0MVX1ezO/QQpqVM6nbJiaDIs1vNK17EmzyOoteinQF+UR2xPV2d
al8jCjEAjQJn4W47X/nDTrHanBmHNUv+3aosTwKBgBaVRzGxb+BMS31hrzFjfXIk
e1o78BjJV5L/J3VYpWLrB4UjcZimzrIuo/rJsJqXjwidhSNeYd14seoeQPieu1SV
hCM1SsBbaaECGTKdYExZYkjsrWtIvtoqlPqedbVv9PCj/ulI8VBs7hbm9iwO3XGQ
6dMUHigDCxvEVJh360tBAoGAKflf8A10vhiRE6oKdDHnf4MVZafSgB+3Bd7oE7Fj
/II44Ol8r+Phuq+dfBnSbMYY6NJ57rVVFmy4luYLaKz5L+LiQUwOv/2NbxS4WdUr
WVw3dViRk6sl+wjdxdqI/JPngeRoEbkTJlk/YQO1iR3/EdfGq6XMbgyTjgi5Yxv0
U/8CgYArvXTFLe+Ia02aTYwIP3eDS9Jxd/Hxk2TpfUf0VDpjBHxqmixYW3hPgzrJ
milaoy6NhM3U0c+8WxGoEZHcQVolPkRdAkCPCzhUNQPSyzRjgSo3mPGNPBFRAre3
Cm9flz88RbOLF/TuSWk/jsh7TtSXt8jL/bR4EdXlwqTFoexpXg==
-----END RSA PRIVATE KEY-----"#.to_string()),
    }
});

static SCENARIO_MISSING_JWK: Lazy<Scenario> = Lazy::new(|| {
    Scenario {
        token: "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6InRlc3RfcHJpdmF0ZV9raWQifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwiaWF0IjoxNTE2MjM5MDIyfQ.VygXMjyLQeUH845pS9aNW5knx_0Kv9KgJ9AZFcYaZFpMMM--Liir7TodF02o9schMCpu0E90foJFtI0aS-JWXqTcVi7Yvp9XzLlLG8P0WNcVeP3eDU-1QoHiDKhvW4Awbvzy-VyqAStvkq_1FCtQMq0qA3F5Y5lXkUFpgcRpaXJ1xzwSNEne16JuGK_Pqt8I00Ps0yI7Mz5VB0LmyLK2l7RGATZ1mZti0Eisak48fAwJqPgjsX6eJkR9I4PfNpb5swIswV_nh1bTLMrCxVWzc4e7mGSEbjP8l2VDQBk3dCMogp7Ep5sMXrEsZCnJIxUdw2xilo0GRY5htxCMPbX10g".to_string(),
        jwk_n: "gRtjwICtIC_4ae33Ks7S80n32PLFEC4UtBanBFE9Pjzcpp4XWDPgbbOkNC9BZ-Jkyq6aoP_UknfJPI-cIvE6IE96bPNGs6DcfZ73Cq2A9ZXTdiuuOiqMwhEgLKFVRUZZ50calENLGyi96-6lcDnwLehh-kEg7ARITmrBO0iAjFU".to_string(),
        kid: "test_private_kid".to_string(),
        public_key: Some(r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAu1SU1LfVLPHCozMxH2Mo
4lgOEePzNm0tRgeLezV6ffAt0gunVTLw7onLRnrq0/IzW7yWR7QkrmBL7jTKEn5u
+qKhbwKfBstIs+bMY2Zkp18gnTxKLxoS2tFczGkPLPgizskuemMghRniWaoLcyeh
kd3qqGElvW/VDL5AaWTg0nLVkjRo9z+40RQzuVaE8AkAFmxZzow3x+VJYKdjykkJ
0iT9wCS0DRTXu269V264Vf/3jvredZiKRkgwlL9xNAwxXFg0x/XFw005UWVRIkdg
cKWTjpBP2dPwVZ4WWC+9aGVd+Gyn1o0CLelf4rEjGoXbAAEgAqeGUxrcIlbjXfbc
mwIDAQAB
-----END RSA PUBLIC KEY-----"#.to_string()),
        private_key: Some(r#"-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQC7VJTUt9Us8cKj
MzEfYyjiWA4R4/M2bS1GB4t7NXp98C3SC6dVMvDuictGeurT8jNbvJZHtCSuYEvu
NMoSfm76oqFvAp8Gy0iz5sxjZmSnXyCdPEovGhLa0VzMaQ8s+CLOyS56YyCFGeJZ
qgtzJ6GR3eqoYSW9b9UMvkBpZODSctWSNGj3P7jRFDO5VoTwCQAWbFnOjDfH5Ulg
p2PKSQnSJP3AJLQNFNe7br1XbrhV//eO+t51mIpGSDCUv3E0DDFcWDTH9cXDTTlR
ZVEiR2BwpZOOkE/Z0/BVnhZYL71oZV34bKfWjQIt6V/isSMahdsAASACp4ZTGtwi
VuNd9tybAgMBAAECggEBAKTmjaS6tkK8BlPXClTQ2vpz/N6uxDeS35mXpqasqskV
laAidgg/sWqpjXDbXr93otIMLlWsM+X0CqMDgSXKejLS2jx4GDjI1ZTXg++0AMJ8
sJ74pWzVDOfmCEQ/7wXs3+cbnXhKriO8Z036q92Qc1+N87SI38nkGa0ABH9CN83H
mQqt4fB7UdHzuIRe/me2PGhIq5ZBzj6h3BpoPGzEP+x3l9YmK8t/1cN0pqI+dQwY
dgfGjackLu/2qH80MCF7IyQaseZUOJyKrCLtSD/Iixv/hzDEUPfOCjFDgTpzf3cw
ta8+oE4wHCo1iI1/4TlPkwmXx4qSXtmw4aQPz7IDQvECgYEA8KNThCO2gsC2I9PQ
DM/8Cw0O983WCDY+oi+7JPiNAJwv5DYBqEZB1QYdj06YD16XlC/HAZMsMku1na2T
N0driwenQQWzoev3g2S7gRDoS/FCJSI3jJ+kjgtaA7Qmzlgk1TxODN+G1H91HW7t
0l7VnL27IWyYo2qRRK3jzxqUiPUCgYEAx0oQs2reBQGMVZnApD1jeq7n4MvNLcPv
t8b/eU9iUv6Y4Mj0Suo/AU8lYZXm8ubbqAlwz2VSVunD2tOplHyMUrtCtObAfVDU
AhCndKaA9gApgfb3xw1IKbuQ1u4IF1FJl3VtumfQn//LiH1B3rXhcdyo3/vIttEk
48RakUKClU8CgYEAzV7W3COOlDDcQd935DdtKBFRAPRPAlspQUnzMi5eSHMD/ISL
DY5IiQHbIH83D4bvXq0X7qQoSBSNP7Dvv3HYuqMhf0DaegrlBuJllFVVq9qPVRnK
xt1Il2HgxOBvbhOT+9in1BzA+YJ99UzC85O0Qz06A+CmtHEy4aZ2kj5hHjECgYEA
mNS4+A8Fkss8Js1RieK2LniBxMgmYml3pfVLKGnzmng7H2+cwPLhPIzIuwytXywh
2bzbsYEfYx3EoEVgMEpPhoarQnYPukrJO4gwE2o5Te6T5mJSZGlQJQj9q4ZB2Dfz
et6INsK0oG8XVGXSpQvQh3RUYekCZQkBBFcpqWpbIEsCgYAnM3DQf3FJoSnXaMhr
VBIovic5l0xFkEHskAjFTevO86Fsz1C2aSeRKSqGFoOQ0tmJzBEs1R6KqnHInicD
TQrKhArgLXX4v3CddjfTRJkFWDbE/CkvKZNOrcf1nhaGCPspRJj2KUkj1Fhl9Cnc
dn/RsYEONbwQSjIfMPkvxF+8HQ==
-----END PRIVATE KEY-----"#.to_string()),
    }
});

#[allow(dead_code)]
#[derive(Clone, Default)]
struct Scenario {
    pub token: String,
    pub jwk_n: String,
    pub kid: String,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
}

#[tokio::test]
async fn should_succeed_with_env() {
    let mock_server = setup_mock_server().await;
    let scenario = SCENARIO_HAPPY_PATH.clone();
    let jwk = Jwk::new(scenario.kid.as_str(), scenario.jwk_n.as_str());

    mock_jwk_issuer(vec![jwk].as_slice())
        .expect(1)
        .mount(&mock_server)
        .await;

    let firebase_auth = FirebaseAuth::builder()
        .env_file("tests/env_files/.env", "FIREBASE_CREDS")
        .jwks_url(TEST_JWKS_URL)
        .build()
        .unwrap();
    let decoded_token = firebase_auth.verify(scenario.token.as_str()).await;

    assert!(decoded_token.is_ok());

    let decoded_token = decoded_token.unwrap();

    assert_eq!(decoded_token.sub, "some-uid");
    assert!(decoded_token.exp > decoded_token.iat);
}

#[tokio::test]
async fn should_succeed_with_env_with_filename() {
    let mock_server = setup_mock_server().await;
    let scenario = SCENARIO_HAPPY_PATH.clone();
    let jwk = Jwk::new(scenario.kid.as_str(), scenario.jwk_n.as_str());

    mock_jwk_issuer(vec![jwk].as_slice())
        .expect(1)
        .mount(&mock_server)
        .await;

    let firebase_auth = FirebaseAuth::builder()
        .env_file("tests/env_files/.env.test", "FIREBASE_CREDS")
        .jwks_url(TEST_JWKS_URL)
        .build()
        .unwrap();
    let decoded_token = firebase_auth.verify(scenario.token.as_str()).await;

    assert!(decoded_token.is_ok());

    let decoded_token = decoded_token.unwrap();

    assert_eq!(decoded_token.sub, "some-uid");
    assert!(decoded_token.exp > decoded_token.iat);
}

#[tokio::test]
async fn should_succeed_with_json_file() {
    let mock_server = setup_mock_server().await;
    let scenario = SCENARIO_HAPPY_PATH.clone();
    let jwk = Jwk::new(scenario.kid.as_str(), scenario.jwk_n.as_str());

    mock_jwk_issuer(vec![jwk].as_slice())
        .expect(1)
        .mount(&mock_server)
        .await;

    let firebase_auth = FirebaseAuth::builder()
        .json_file("tests/env_files/firebase-creds.json")
        .jwks_url(TEST_JWKS_URL)
        .build()
        .unwrap();
    let decoded_token = firebase_auth.verify(scenario.token.as_str()).await;

    assert!(decoded_token.is_ok());

    let decoded_token = decoded_token.unwrap();

    assert_eq!(decoded_token.sub, "some-uid");
    assert!(decoded_token.exp > decoded_token.iat);
}

#[tokio::test]
async fn missing_kid() {
    let token_without_kid = SCENARIO_MISSING_KID.clone().token;
    let decoded_token =
        FirebaseAuth::default().verify(&token_without_kid).await;

    assert!(decoded_token.is_err());
    assert!(matches!(
        decoded_token.err().unwrap(),
        Error::InvalidJwt(InvalidJwt::MissingKid)
    ));
}

// Test for when the JWK issuer return empty list
#[tokio::test]
async fn missing_jwk() {
    let mock_server = setup_mock_server().await;
    let scenario = SCENARIO_MISSING_JWK.clone();

    // JWK issue returns empty list of jwks
    mock_jwk_issuer(Vec::new().as_slice())
        .expect(1)
        .mount(&mock_server)
        .await;

    let decoded_token = FirebaseAuth::builder()
        .jwks_url(TEST_JWKS_URL)
        .build()
        .unwrap()
        .verify(scenario.token.as_str())
        .await;

    assert!(decoded_token.is_err());
    assert!(matches!(
        decoded_token.err().unwrap(),
        Error::InvalidJwt(InvalidJwt::MatchingJwkNotFound)
    ))
}
