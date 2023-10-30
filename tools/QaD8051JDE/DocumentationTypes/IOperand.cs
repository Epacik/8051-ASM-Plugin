namespace DocumentationTypes;

public interface IOperand<TEnum>
    where TEnum : struct, IConvertible
{
    TEnum Operand { get; set; }
    TEnum WhenFirstIs { get; set; }
}
