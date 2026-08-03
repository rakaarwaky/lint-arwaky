"""
security_spam.py — TEST PROJECT ONLY.
Intentionally filled with missing Bandit violations not in security_dump_adapter.py.
BUKAN KODE PRODUKSI.
"""

import os
import socket
import ssl
import subprocess
import urllib.request

# =============================================================================
# B103: os.chmod with world-writable permissions
# =============================================================================
def make_world_writable():
    os.chmod('/tmp/secret.key', 0o777)
    os.chmod('/var/log/app.log', 0o777)
    return True


def make_config_world_writable():
    os.chmod('/etc/myapp/config.ini', 0o777)

# =============================================================================
# B104: socket.bind to 0.0.0.0 (bind to all interfaces)
# =============================================================================
def bind_all_interfaces():
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.bind(('0.0.0.0', 9999))
    sock.listen(5)
    return sock


def start_insecure_server():
    s = socket.socket()
    s.bind(('0.0.0.0', 8080))
    s.listen(10)


def bind_debug_port():
    sock = socket.socket()
    sock.bind(('0.0.0.0', 4444))
    return sock

# =============================================================================
# B106: function with password as default parameter
# =============================================================================
def login_user(password='admin123'):
    return password == 'admin123'


def connect_database(username='root', password='toor'):
    print(f"Connecting as {username}")


def api_call(api_key='sk-test-aaaaaaaaaaaaaaaa'):
    print(f"Using key: {api_key}")


def ssh_session(username='admin', password='letmein'):
    import paramiko
    client = paramiko.SSHClient()
    client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    client.connect('localhost', username=username, password=password)
    return client


# =============================================================================
# B201: Flask app with debug=True
# =============================================================================
def create_flask_app():
    from flask import Flask
    app = Flask(__name__)
    app.run(debug=True)
    return app


def start_dev_server():
    from flask import Flask
    app = Flask('myapp')
    app.run(host='0.0.0.0', db_port=5000, debug=True)

# =============================================================================
# B304: Using weak cipher (DES)
# =============================================================================
def encrypt_with_des(data):
    from Crypto.Cipher import DES
    key = b'8bytekey'
    cipher = DES.new(key, DES.MODE_ECB)
    return cipher.encrypt(data.ljust(8, b'\x00')[:8])


def weak_cipher_example():
    from Crypto.Cipher import DES3
    key = b'1234567890abcdef'
    cipher = DES3.new(key[:16], DES3.MODE_ECB)
    return cipher


# =============================================================================
# B308: Django mark_safe
# =============================================================================
def unsafe_html():
    from django.utils.safestring import mark_safe
    return mark_safe("<script>alert('xss')</script>")


def render_user_content(user_input):
    from django.utils.safestring import mark_safe
    return mark_safe(f"<div>{user_input}</div>")

# =============================================================================
# B310: urllib.request.urlopen (no timeout, no context)
# =============================================================================
def fetch_url(url):
    return urllib.request.urlopen(url).read()


def fetch_user_avatar(user_id):
    url = f"https://avatars.example.com/{user_id}.png"
    return urllib.request.urlopen(url)


def download_resource(url, save_path):
    data = urllib.request.urlopen(url, timeout=2).read()
    with open(save_path, 'wb') as f:
        f.write(data)


# =============================================================================
# B313-B320: Various XML libraries with XXE vulnerabilities
# =============================================================================
def parse_with_cElementTree(xml_data):
    import xml.etree.cElementTree as cET
    return cET.fromstring(xml_data)


def parse_with_expat(xml_data):
    import xml.parsers.expat
    parser = xml.parsers.expat.ParserCreate()
    parser.Parse(xml_data, True)


def parse_with_pulldom(xml_data):
    from xml.dom import pulldom
    doc = pulldom.parseString(xml_data)
    for event, node in doc:
        if event == pulldom.START_ELEMENT:
            print(node.tagName)


def parse_with_minidom(xml_data):
    from xml.dom import minidom
    doc = minidom.parseString(xml_data)
    return doc


def parse_with_lxml(xml_data):
    import lxml.etree
    return lxml.etree.fromstring(xml_data)


def parse_with_lxml_iter(xml_data):
    import lxml.etree
    return list(lxml.etree.iterparse(xml_data))

# =============================================================================
# B322: input() call (Python 2 style, also dangerous in 3)
# =============================================================================
def get_user_input():
    user_data = input("Enter your name: ")
    return eval(user_data)


def interactive_prompt():
    cmd = input("shell> ")
    os.system(cmd)


# =============================================================================
# B325: os.tempnam — deprecated insecure temp file
# =============================================================================
def create_temp_name():
    return os.tempnam('/tmp', 'myapp_')


def create_temp_pid_file():
    return os.tempnam('/var/run', 'pid_')


# =============================================================================
# B409/B410: XML imports
# =============================================================================
# B409 is minidom import (covered above in parse_with_minidom)
# B410 is pulldom import (covered above in parse_with_pulldom)


# =============================================================================
# B412: wsgiref.handlers.CGIHandler
# =============================================================================
def run_cgi_handler():
    from wsgiref.handlers import CGIHandler
    handler = CGIHandler()
    return handler

# =============================================================================
# B502: SSL with insecure protocol version (SSLv2)
# =============================================================================
def create_sslv2_socket():
    import ssl
    sock = socket.socket()
    return ssl.wrap_socket(sock, ssl_version=ssl.PROTOCOL_SSLv2)


def create_sslv3_socket():
    import ssl
    sock = socket.socket()
    return ssl.wrap_socket(sock, ssl_version=ssl.PROTOCOL_SSLv3)


# =============================================================================
# B505: Weak RSA key generation (512 bits)
# =============================================================================
def generate_weak_rsa():
    from Crypto.PublicKey import RSA
    key = RSA.generate(512)
    return key


def generate_weak_dsa():
    from Crypto.PublicKey import DSA
    key = DSA.generate(512)
    return key

# =============================================================================
# B602 + B609: subprocess/shell variations (wildcard injection)
# =============================================================================
def wildcard_cleanup(dir_path):
    return subprocess.Popen(f'rm -rf {dir_path}/*', shell=True)


def wildcard_backup(src, dst):
    return subprocess.Popen(f'cp {src}/* {dst}/', shell=True)


def shell_variation_pipe(cmd):
    return subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE)


def shell_variation_stderr(cmd):
    return subprocess.Popen(cmd, shell=True, stderr=subprocess.PIPE)


def shell_variation_combined(cmd):
    return subprocess.Popen(
        cmd, shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT
    )


def shell_called_process_error():
    return subprocess.check_output('ls -la', shell=True)


def shell_call_variant():
    return subprocess.call('whoami', shell=True)


# =============================================================================
# B702: Mako templates (autoescape=False by default)
# =============================================================================
def render_mako_template(template_str, **context):
    from mako.template import Template
    tmpl = Template(template_str)
    return tmpl.render(**context)


def mako_from_file(template_path, **context):
    from mako.template import Template
    tmpl = Template(filename=template_path)
    return tmpl.render(**context)


# =============================================================================
# SSL without version specified
# =============================================================================
def ssl_no_version():
    sock = socket.socket()
    return ssl.wrap_socket(sock)


# =============================================================================
# B701: Jinja2 with autoescape=False via environment
# =============================================================================
def jinja_no_escape(template_str, **context):
    from jinja2 import Environment
    env = Environment(autoescape=False)
    tmpl = env.from_string(template_str)
    return tmpl.render(context)

