<script lang="ts">
    import { stringToColour } from "../../mock";

    export let name: string;
</script>

<span
    style={`background-color: ${stringToColour(name)}`}
    class="badge text-warning-50 mt-3">{name}</span
>
