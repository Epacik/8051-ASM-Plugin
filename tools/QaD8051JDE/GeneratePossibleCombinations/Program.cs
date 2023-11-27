

using DocumentationTypes;
using DocumentationTypes.PartialDocumentationElement;
using System.Reflection;
using System.Reflection.Metadata.Ecma335;
using System.Text.Json;
using System.Text.Json.Serialization;

var assembly = Assembly.GetExecutingAssembly();
var assemblyPath = assembly.Location;
var repoPath = assemblyPath.Substring(
    0, 
    assemblyPath.LastIndexOf("tools"));

var docsPath = Path.Combine(repoPath, "server", "asm8051_docs", "json_documentation");
var sharedPath = Path.Combine(docsPath, ".shared");
var snglishPath = Path.Combine(docsPath, "english");

string[] filesWithInstructions =
{
    "arithmetic.json", "data_transfer.json", "program_control.json", "logical.json"
};

var options = new JsonSerializerOptions
{
    WriteIndented = true,
    Converters =
        {
            new JsonStringEnumConverter(JsonNamingPolicy.CamelCase),
        }
};

var files = Directory.GetFiles(".");

List<string> validInstructionsCombinations = new();

foreach (var file in filesWithInstructions.Select(x => Path.Join(sharedPath, x)))
{
    var contents = File.ReadAllText(file);
    var instructions = JsonSerializer.Deserialize<Dictionary<string, SharedDocumentationElement>>(contents, options);

    foreach (var (instruction, data) in instructions!)
    {
        var lines = data.ValidOperands!.Count switch
        {
            0 => new List<string>(),
            1 => InstructionLinesForOneOperand(instruction, data.ValidOperands),
            2 => InstructionLinesForTwoOperands(instruction, data.ValidOperands),
            3 => InstructionLinesForThreeOperands(instruction, data.ValidOperands),
            _ => throw new IndexOutOfRangeException(),
        };
        validInstructionsCombinations.AddRange(lines);
        validInstructionsCombinations.Add("");
    }
}

var content = string.Join("\n", validInstructionsCombinations);
Console.WriteLine(content);
File.WriteAllText("./out.txt", content);

IEnumerable<string> InstructionLinesForOneOperand(string instruction, List<List<ValidOperand>> validOperands)
{
    foreach (var validOperand in validOperands[0])
    {
        for (int i = 0; i < validOperand.Operand.TokenMaxValue(); i++)
        {
            var token0 = validOperand.Operand.ToToken(i);
            yield return $"    {instruction} {token0}";
            yield return $"LABEL {instruction} {token0}";
            yield return $"LABEL: {instruction} {token0}";
        }
    }
}

IEnumerable<string> InstructionLinesForTwoOperands(string instruction, List<List<ValidOperand>> validOperands)
{
    foreach (var validOperand0 in validOperands[0])
    {
        foreach (var validOperand1 in validOperands[1])
        {
            for (int i = 0; i < validOperand0.Operand.TokenMaxValue(); i++)
            {
                for (int n = 0; n < validOperand1.Operand.TokenMaxValue(); n++)
                {
                    var token0 = validOperand0.Operand.ToToken(i);
                    var token1 = validOperand1.Operand.ToToken(n);
                    yield return $"    {instruction} {token0},{token1}";
                    yield return $"    {instruction} {token0}, {token1}";
                    yield return $"LABEL {instruction} {token0},{token1}";
                    yield return $"LABEL {instruction} {token0}, {token1}";
                    yield return $"LABEL: {instruction} {token0},{token1}";
                    yield return $"LABEL: {instruction} {token0}, {token1}";
                }
            }
        }
    }
}

IEnumerable<string> InstructionLinesForThreeOperands(string instruction, List<List<ValidOperand>> validOperands)
{
    foreach (var validOperand0 in validOperands[0])
    {
        foreach (var validOperand1 in validOperands[1])
        {
            foreach (var validOperand2 in validOperands[2])
            {
                for (int i = 0; i < validOperand0.Operand.TokenMaxValue(); i++)
                {
                    for (int n = 0; n < validOperand1.Operand.TokenMaxValue(); n++)
                    {
                        for (int k = 0; k < validOperand2.Operand.TokenMaxValue(); k++)
                        {
                            var token0 = validOperand0.Operand.ToToken(i);
                            var token1 = validOperand1.Operand.ToToken(n);
                            var token2 = validOperand2.Operand.ToToken(k);
                            yield return $"    {instruction} {token0},{token1},{token2}";
                            yield return $"    {instruction} {token0},{token1}, {token2}";
                            yield return $"    {instruction} {token0}, {token1},{token2}";
                            yield return $"    {instruction} {token0}, {token1}, {token2}";

                            yield return $"LABEL {instruction} {token0},{token1},{token2}";
                            yield return $"LABEL {instruction} {token0},{token1}, {token2}";
                            yield return $"LABEL {instruction} {token0}, {token1},{token2}";
                            yield return $"LABEL {instruction} {token0}, {token1}, {token2}";
                            yield return $"LABEL: {instruction} {token0},{token1},{token2}";
                            yield return $"LABEL: {instruction} {token0},{token1}, {token2}";
                            yield return $"LABEL: {instruction} {token0}, {token1},{token2}";
                            yield return $"LABEL: {instruction} {token0}, {token1}, {token2}";
                        }
                    }
                }
            }
        }
    }
}
