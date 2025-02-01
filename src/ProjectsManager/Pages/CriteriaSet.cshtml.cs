using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using ProjectsManager.Models;
using ProjectsManager.Services;

namespace ProjectsManager.Pages;

public class CriteriaModel(ProjectsApiService apiService) : PageModel
{
    private readonly ProjectsApiService _apiService = apiService;

    [BindProperty]
    public Criteria Criteria { get; set; } = new();
    [BindProperty]
    public Criterion NewCriterion { get; set; } = new();

    [BindProperty]
    public int? SelectedSetId { get; set; }

    public List<Criteria> CriteriaSets { get; set; } = [];
    public string ApiMessage { get; set; } = string.Empty;

    public async Task OnGet()
    {
        CriteriaSets = await _apiService.GetAllCriteriaSetsAsync();
    }

    public async Task<IActionResult> OnPostAsync()
    {
        if (!ModelState.IsValid)
        {
            return Page();
        }

        bool success = await _apiService.CreateCriteriaAsync(Criteria);
        if (success)
        {
            TempData["SuccessMessage"] = "Criteria set successfully created!";
            return RedirectToPage("/CriteriaSet");
        }
        else
        {
            ModelState.AddModelError(string.Empty, "Failed to create criteria.");
            CriteriaSets = await _apiService.GetAllCriteriaSetsAsync();
            return Page();
        }
    }

    public async Task<IActionResult> OnPostAddCriterionAsync()
    {
        if (SelectedSetId.HasValue)
        {
            bool success = await _apiService.AddCriterionToSetAsync(SelectedSetId.Value, NewCriterion);
            if (success)
                TempData["SuccessMessage"] = "Criterion successfully added!";
            else
                TempData["ErrorMessage"] = "Failed to add criterion.";
        }
        else
        {
            Criteria.Set.Add(NewCriterion);
            bool success = await _apiService.CreateCriteriaAsync(Criteria);
            if (success)
                TempData["SuccessMessage"] = "New criteria set with criterion successfully created!";
            else
                TempData["ErrorMessage"] = "Failed to create new criteria set.";
        }

        return RedirectToPage("/CriteriaSet");
    }

    public async Task<IActionResult> OnPostDeleteCriterionAsync(int setId, string criterionName)
    {
        bool success = await _apiService.DeleteCriterionAsync(setId, criterionName);
        if (success)
        {
            TempData["SuccessMessage"] = "Criterion successfully deleted!";
        }
        else
        {
            TempData["ErrorMessage"] = "Failed to delete criterion.";
        }

        return RedirectToPage("/CriteriaSet");
    }

}
