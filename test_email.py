import time

import email
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
import smtplib

HOST = 'REDACTED_SMTP_HOST'
USER = 'notifiertest'
PASS = 'REDACTED_PASSWORD'

if __name__ == '__main__':
    msg = MIMEMultipart('alternative')
    msg['Subject'] = 'roundtrip test'
    msg['From'] = 'notifiertest@updatepromise.com'
    msg['To'] = 'jordan.gonzalez@corp.updatepromise.com'
    msg['Date'] = email.utils.formatdate()
    #msg['Message-ID'] = email.utils.make_msgid(msgid)
    msg.attach(MIMEText('hello', 'plain', _charset='utf-8'))

    server = smtplib.SMTP()
    server.set_debuglevel(1)
    server.connect(HOST, 587)
    server.login(USER, PASS)
    msg_string = msg.as_string()
    server.sendmail(msg['From'], msg['To'], msg_string)