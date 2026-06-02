"""
Security Dump Adapter — TEST PROJECT ONLY.
Sengaja dibuat sangat tidak aman untuk menguji Bandit scanner.
BUKAN KODE PRODUKSI.
"""

import os
import subprocess
import tempfile
import pickle
import hashlib
import random
import telnetlib
import ftplib
import xml.etree.ElementTree as ET
import requests
import yaml
import json
import sqlite3
import base64
from jinja2 import Template, Environment

# =============================================================================
# VIOLATION 1: hardcoded password string literal (B105, B106, B107)
# =============================================================================
DB_PASSWORD = "super_secret_password_123!"
API_SECRET = "sk-live-abcdefghijklmnopqrstuvwxyz"
AWS_SECRET_KEY = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
DB_CONNECTION_STRING = "postgresql://admin:passw0rd@localhost:5432/prod"


# =============================================================================
# VIOLATION 2: assert statement (B101)
# =============================================================================
def validate_user(token):
    assert token is not None, "Token cannot be None"
    assert len(token) > 0, "Token cannot be empty"
    return True


def check_admin(user_id):
    assert user_id == 1, "Only admin can access"
    return True


# =============================================================================
# VIOLATION 3: eval() dan exec() (B307)
# =============================================================================
def evaluate_expression(expr):
    return eval(expr)


def execute_code(code):
    exec(code)


def eval_dynamic(user_input):
    result = eval(f"lambda x: {user_input}")
    return result(42)


def exec_script(script_text):
    exec_globals = {"__builtins__": __builtins__}
    exec(script_text, exec_globals)


# =============================================================================
# VIOLATION 4: insecure temp file — mktemp (B108)
# =============================================================================
def write_temp_data(data):
    tmp_path = tempfile.mktemp()
    with open(tmp_path, "w") as f:
        f.write(data)
    print(f"Data written to {tmp_path}")
    return tmp_path


def create_temp_config():
    tmp = tempfile.mktemp(suffix=".cfg", prefix="tmp_")
    os.system(f"echo '[config]' > {tmp}")
    return tmp


# =============================================================================
# VIOLATION 5: try/except pass — bare pass (B110)
# =============================================================================
def silent_fail():
    try:
        1 / 0
    except:
        pass


def parse_user_data(data):
    try:
        return json.loads(data)
    except:
        pass


def database_operation(query):
    try:
        conn = sqlite3.connect(":memory:")
        conn.execute(query)
        conn.close()
    except:
        pass


# =============================================================================
# VIOLATION 6: Pickle module import dan penggunaan (B301)
# =============================================================================
def serialize_object(obj):
    return pickle.dumps(obj)


def deserialize_data(data):
    return pickle.loads(data)


def load_pickled_user(data):
    return pickle.loads(base64.b64decode(data))


# =============================================================================
# VIOLATION 7 + 15: MD5 and SHA1 hash usage (B303)
# =============================================================================
def hash_password_md5(password):
    return hashlib.md5(password.encode()).hexdigest()


def hash_data_sha1(data):
    return hashlib.sha1(data.encode()).hexdigest()


def verify_checksum(content, checksum):
    md5_hash = hashlib.md5(content.encode()).hexdigest()
    return md5_hash == checksum


def hash_file_sha1(filepath):
    h = hashlib.sha1()
    with open(filepath, "rb") as f:
        h.update(f.read())
    return h.hexdigest()


# =============================================================================
# VIOLATION 8: random module — bukan crypto random (B311)
# =============================================================================
def generate_session_id():
    return random.randint(100000, 999999)


def generate_token():
    chars = "abcdefghijklmnopqrstuvwxyz0123456789"
    token = "".join(random.choice(chars) for _ in range(32))
    return token


def shuffle_api_keys(keys):
    random.shuffle(keys)
    return keys


def pick_backup_node(nodes):
    return random.choice(nodes)


# =============================================================================
# VIOLATION 9: yaml.load tanpa safe_load (B506)
# =============================================================================
def load_yaml_config(content):
    return yaml.load(content)


def parse_deployment_yaml(filepath):
    with open(filepath, "r") as f:
        return yaml.load(f)


def merge_yaml_configs(base, override):
    merged = yaml.load(base)
    overrides = yaml.load(override)
    merged.update(overrides)
    return merged


# =============================================================================
# VIOLATION 10: subprocess dengan shell=True (B602)
# =============================================================================
def run_command(cmd):
    return subprocess.check_output(cmd, shell=True)


def execute_shell(script):
    proc = subprocess.Popen(script, shell=True, stdout=subprocess.PIPE)
    return proc.communicate()[0]


def ping_host(host):
    return subprocess.call(f"ping -c 1 {host}", shell=True)


def background_job(job_name):
    subprocess.Popen(f"nohup {job_name} &", shell=True)

# =============================================================================
# VIOLATION 11: SQL injection via string concatenation (B608)
# =============================================================================
def get_user_by_id(user_id):
    conn = sqlite3.connect("users.db")
    query = f"SELECT * FROM users WHERE id = {user_id}"
    conn.execute(query)
    conn.close()


def login_user(username, password):
    conn = sqlite3.connect("users.db")
    query = f"SELECT * FROM users WHERE username = '{username}' AND password = '{password}'"
    cursor = conn.execute(query)
    return cursor.fetchone()


def search_products(term):
    conn = sqlite3.connect("shop.db")
    query = "SELECT * FROM products WHERE name LIKE '%" + term + "%'"
    return conn.execute(query).fetchall()


def delete_user(username):
    conn = sqlite3.connect("admin.db")
    conn.execute("DELETE FROM users WHERE username = '" + username + "'")
    conn.commit()
    conn.close()


# =============================================================================
# VIOLATION 12: os.system, os.popen (B605)
# =============================================================================
def run_system_cmd(command):
    os.system(command)


def list_directory(path):
    os.system(f"ls -la {path}")


def make_backup(src, dst):
    os.popen(f"cp -r {src} {dst}")


def kill_process(pid):
    os.system(f"kill -9 {pid}")


def tail_log(logfile):
    output = os.popen(f"tail -100 {logfile}").read()
    return output


# =============================================================================
# VIOLATION 13: requests dengan verify=False (B501)
# =============================================================================
def fetch_insecure(url):
    return requests.get(url, verify=False)


def post_data(endpoint, payload):
    return requests.post(endpoint, json=payload, verify=False)


def download_cert(cert_url):
    resp = requests.get(cert_url, verify=False, timeout=5)
    with open("/tmp/cert.pem", "wb") as f:
        f.write(resp.content)


def health_check_insecure(host):
    try:
        r = requests.get(f"https://{host}/health", verify=False, timeout=2)
        return r.status_code == 200
    except:
        pass


# =============================================================================
# VIOLATION 14: ftplib, telnetlib import (B321)
# =============================================================================
def ftp_upload(host, username, password, local_path, remote_path):
    ftp = ftplib.FTP(host)
    ftp.login(username, password)
    with open(local_path, "rb") as f:
        ftp.storbinary(f"STOR {remote_path}", f)
    ftp.quit()


def telnet_connect(host, port):
    tn = telnetlib.Telnet(host, port)
    tn.read_until(b"login: ")
    tn.write(b"admin\n")
    tn.read_until(b"Password: ")
    tn.write(b"password\n")
    return tn


def ftp_download_all(host, username, password):
    ftp = ftplib.FTP(host)
    ftp.login(username, password)
    files = ftp.nlst()
    for f in files:
        with open(f, "wb") as local:
            ftp.retrbinary(f"RETR {f}", local.write)
    ftp.quit()


# =============================================================================
# VIOLATION 16: Jinja2 autoescape=False (B704)
# =============================================================================
def render_template(template_str, context):
    env = Environment(autoescape=False)
    template = env.from_string(template_str)
    return template.render(context)


def render_user_profile(user_input):
    tpl = Template("<html><body>" + user_input + "</body></html>")
    tpl.globals["autoescape"] = False
    return tpl.render(name="test")


def generate_email_body(user_name, message):
    env = Environment(autoescape=False)
    template = env.from_string(
        "<p>Hello {{ name }}</p><p>{{ message }}</p>"
    )
    return template.render(name=user_name, message=message)


# =============================================================================
# VIOLATION 17: XML parsing dengan vulnerable parser (B314, B315, B319)
# =============================================================================
def parse_xml(xml_string):
    return ET.fromstring(xml_string)


def parse_xml_file(filepath):
    tree = ET.parse(filepath)
    return tree.getroot()


def extract_from_xml(xml_data, xpath):
    root = ET.fromstring(xml_data)
    return root.findall(xpath)


# =============================================================================
# BONUS VIOLATIONS
# =============================================================================

# B302: marshal deserialization
import marshal


def load_compiled(code_bytes):
    return marshal.loads(code_bytes)


# B303: insecure cipher (use of weak cryptography)
def weak_encrypt(data):
    return base64.b64encode(data.encode())


# B401: import subprocess with shell injection wrapper
def run_with_pipe(cmds):
    full_cmd = " | ".join(cmds)
    return os.popen(full_cmd).read()


# B402: import ftplib (already above)
# B403: import pickle (already above)
# B404: import subprocess (already above)
# B405: start_process with shell=True — already covered
# B406: import xml.etree (already above)
# B407: import xml.dom.minidom
import xml.dom.minidom


def parse_xml_dom(data):
    return xml.dom.minidom.parseString(data)


# B408: import xml.sax
import xml.sax


def sax_parse(xml_file):
    xml.sax.parse(xml_file)


# B409: import xml.etree (already)

# B410: import lxml
try:
    import lxml.etree as lxml_etree

    def lxml_parse(data):
        return lxml_etree.fromstring(data)
except ImportError:
    pass

# B411: import xmlrpclib
import xmlrpc.client as xmlrpclib


def xmlrpc_call(url, method, *args):
    proxy = xmlrpclib.ServerProxy(url)
    return getattr(proxy, method)(*args)


# B412: import http.client with SSL disabled
import http.client


def http_call_insecure(host, path):
    conn = http.client.HTTPConnection(host)
    conn.request("GET", path)
    return conn.getresponse()


# B413: import paramiko without host key verification
try:
    import paramiko

    def ssh_connect_no_key(host, username, password):
        client = paramiko.SSHClient()
        client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
        client.connect(host, username=username, password=password)
        return client
except ImportError:
    pass

# B501: request with verify=False already covered

# B502: SSL with bad SNI / no cert validation
import ssl


def create_bad_ssl_context():
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE
    return ctx


# B503: SSL with bad protocol version
def create_ssl_v2_context():
    ctx = ssl.SSLContext(ssl.PROTOCOL_TLSv1)
    return ctx


# B504: SSL with no version
def ssl_no_version():
    return ssl.wrap_socket


# B505: weak cryptography key
WEAK_RSA_KEY = "-----BEGIN RSA PRIVATE KEY-----\nMIIBOgIBAAJBAKj34GkxFhD90vcNLYLcMXMGBWbR0iYIKdN9\n-----END RSA PRIVATE KEY-----"


# B601: paramiko call (already above)

# B603: subprocess without shell=False — already covered in run_command with shell=True

# B604: any_other_function_with_shell_equals_true
def run_git_command(repo_path):
    return subprocess.check_output("git status", shell=True, cwd=repo_path)


# B605: start_process with a shell (os.system, os.popen covered)

# B606: start_process with shell injection
def grep_file(pattern, filepath):
    return os.system(f"grep '{pattern}' '{filepath}'")


# B607: start_process with partial path
def call_custom_binary():
    return subprocess.check_output("my_custom_tool --version", shell=True)


# B608: SQL injection — already covered

# B609: Linux commands with wildcard injection
def cleanup_temp():
    os.system("rm -rf /tmp/*")


# B610: django render with autoescape — N/A

# B611: django var with autoescape — N/A

# B701: jinja2 with dangerous expression — covered

# B702: use of mako templates — skip

# B703: use of django HTML rendering — N/A

# =============================================================================
# FINAL: A function that chains many violations together
# =============================================================================
def do_everything_wrong(user_input, user_id, db_row):
    """The ultimate insecure function — triggers most Bandit rules at once."""
    # eval
    result = eval(user_input)
    # exec
    exec("import os; os.system('echo pwned')")
    # hardcoded password
    pwd = "letmein123"
    # md5
    h = hashlib.md5(pwd.encode()).hexdigest()
    # random
    tok = random.randint(0, 999999)
    # pickle
    data = pickle.dumps({"user": user_id, "token": tok})
    # sql injection
    conn = sqlite3.connect(":memory:")
    conn.execute(f"SELECT * FROM users WHERE id = {user_id}")
    # subprocess shell
    subprocess.call(f"echo {user_input}", shell=True)
    # os.system
    os.system("whoami")
    # try/except pass
    try:
        1 / 0
    except:
        pass
    # yaml.load
    yaml.load("a: 1")
    # requests verify=False
    requests.get("https://evil.com", verify=False)
    # insecure temp
    tmp = tempfile.mktemp()
    # assert
    assert user_id > 0
    return result
