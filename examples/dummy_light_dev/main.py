import sys
from cyclonedds.domain import DomainParticipant
from cyclonedds.pub import DataWriter
from cyclonedds.sub import DataReader
from cyclonedds.topic import Topic
from threading import Thread

import threading
import LightState
import TurnLight
import time

light = False
l = threading.Lock()

def receive():
    global light
    with l:
        participant = DomainParticipant()
        topic = Topic(participant, "/rt/piccolo/Turn_Light", TurnLight.DataType)
        reader = DataReader(participant, topic)

    while True:
        sample = reader.take(10)
        if len(sample) > 0:
            l.acquire()
            light = sample[0].operation=="on"
            print(light)
            l.release()
        time.sleep(0.2)

def send():
    global light
    with l:
        participant = DomainParticipant()
        topic = Topic(participant, "/rt/piccolo/Light_State", LightState.DataType)
        writer = DataWriter(participant, topic)

    time.sleep(2)

    count = 0
    while True:
        l.acquire()
        data = LightState.DataType(light)
        l.release()
        if count % 5 == 0:
            print("send")
        writer.write(data)
        time.sleep(0.2)
        count += 1
        count %= 5


if __name__ == '__main__':
    rec = Thread(target=send)
    sen = Thread(target=receive)

    rec.start()
    sen.start()

    rec.join()
    sen.join()
