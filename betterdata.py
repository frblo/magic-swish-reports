import io

def rawToCSV():
    filename = "data.csv"
    initialiseFile(filename)
    
    wf = io.open(filename, 'a')



def initialiseFile(filename: str):
    wf = io.open(filename, 'w')
    wf.write("")
    wf.close()

def writeToFile(filename: str, data: str, wf: io.TextIOWrapper):
    wf.write(data)

def readData(filename: str) -> str:
    rf = io.open(filename, 'r')
    


# def readLine(filename: str, rf: io.TextIOWrapper) -> str:
#     rf.readline()
