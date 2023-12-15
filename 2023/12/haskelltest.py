import subprocess

for line in open("./input2"):
    chars = line.split(" ")[0]
    groups = [int(x) for x in line.split(" ")[1].split(",")]
    regex1 = "\.*" + ("".join([f"#{'{'}{number}{'}'}\.+" for number in groups]))[:-3] + "\.*"
    regex2 = chars.replace("?", "[#.]").replace(".", "\.")
    # docker exec -it 055a5629a751 "/root/.cabal/bin/genex"
    command = f"/root/.cabal/bin/genex \"{regex1}\" \"{regex2}\" | wc -l"
    print(command)
    found = subprocess.check_output(["docker", "exec", "-it", "055a5629a751", "bash", "-c", command])
    print(line, regex1, regex2, found)